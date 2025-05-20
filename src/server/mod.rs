use crate::{context, error, processors};
use axum;
use bon;
use color_eyre::eyre::{self, WrapErr};
use hotwatch;
use std::{path, sync::Arc};
use tokio::{self, sync::RwLock};
use tower_livereload;
use tracing;

pub mod handlers;
#[cfg(test)]
mod tests;

pub const ADDRESS: std::net::Ipv4Addr = std::net::Ipv4Addr::new(0, 0, 0, 0);

#[derive(Debug, Clone)]
pub struct WebState {
    pub content_path: String,
    pub output_path: String,
}
#[bon::bon]
impl WebState {
    #[builder]
    pub fn new(content_path: String, output_path: String) -> eyre::Result<WebState, error::Error> {
        return Ok(WebState {
            content_path,
            output_path,
        });
    }
}

#[bon::builder]
pub async fn serve(ctx: &mut context::Context) -> eyre::Result<(), error::Error> {
    let content_folder = ctx.build_args.content.clone();
    let output_folder = ctx.build_args.output.clone();

    let web_state = Arc::new(RwLock::new(
        WebState::builder()
            .content_path(content_folder.clone())
            .output_path(output_folder.clone())
            .build()
            .wrap_err("failed to create web state")?,
    ));

    let live_reload_layer = tower_livereload::LiveReloadLayer::new();
    let reloader = live_reload_layer.reloader();

    let router: axum::Router = axum::Router::new()
        .route("/", axum::routing::get(handlers::index_route))
        .route("/*filepath", axum::routing::get(handlers::main_route))
        .route(
            "/favicon.ico",
            axum::routing::get(|| async { axum::http::StatusCode::NO_CONTENT }),
        )
        .with_state(web_state)
        .layer(live_reload_layer);

    let mut watcher =
        hotwatch::Hotwatch::new_with_custom_delay(std::time::Duration::from_millis(100))
            .wrap_err("failed to create a hotwatch instance with custom delay")?;
    let mut watcher_ctx = ctx.clone();
    watcher
        .watch(
            content_folder.clone(),
            move |event: hotwatch::Event| match event.kind {
                hotwatch::EventKind::Modify(hotwatch::notify::event::ModifyKind::Data(_))
                | hotwatch::EventKind::Create(_) => {
                    watcher_ctx.is_rebuild = true;
                    watcher_ctx.build_path = event.paths[0].to_string_lossy().to_string();
                    if let Some(rebuild_file_name) =
                        path::Path::new(&watcher_ctx.build_path).extension()
                    {
                        if rebuild_file_name.to_string_lossy().to_string() == "md" {
                            let parsed_files =
                                match processors::parse().ctx(&mut watcher_ctx).call() {
                                    Ok(safe_processed_files) => safe_processed_files,
                                    Err(e) => {
                                        tracing::error!(
                                            "failed to parse content files, Error: {:#?}",
                                            e.to_string()
                                        );
                                        std::process::exit(1);
                                    }
                                };
                            match processors::emit()
                                .ctx(&mut watcher_ctx)
                                .parsed_files(&parsed_files)
                                .call()
                            {
                                Ok(_) => {}
                                Err(e) => {
                                    tracing::error!(
                                        "failed to emit processed content files, Error: {:#?}",
                                        e.to_string()
                                    );
                                    std::process::exit(1);
                                }
                            };
                            reloader.reload();
                        }
                    }
                }
                hotwatch::EventKind::Remove(_) => {}
                _ => {}
            },
        )
        .wrap_err("failed to watch for changes using hotwatch")?;

    let server_port = ctx.config.server.port.clone();
    let listener = tokio::net::TcpListener::bind(ADDRESS.to_string() + ":" + &server_port)
        .await
        .wrap_err("failed to bind TCP Listener to address")?;

    if ctx.config.settings.logging == true {
        tracing::info!("running on {}:{}", ADDRESS, server_port);
    }

    axum::serve(listener, router)
        .await
        .wrap_err("Failed to start the serrver listener")?;

    return Ok(());
}
