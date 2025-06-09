use crate::{context, error, helpers, processors, utils};
use axum;
use bon;
use color_eyre::eyre::{self, WrapErr};
use std::{path, sync::Arc};
use tokio::{self, sync::RwLock};
use tower_livereload;
use tracing;

pub mod handlers;
#[cfg(test)]
pub mod tests;

pub const ADDRESS: std::net::Ipv4Addr = std::net::Ipv4Addr::new(0, 0, 0, 0);

#[derive(Debug, Clone, bon::Builder)]
pub struct WebState {
    pub content_path: vfs::VfsPath,
    pub output_path: vfs::VfsPath,
}

#[bon::builder]
pub async fn serve(ctx: &context::Context) -> eyre::Result<(), error::Error> {
    let content_folder = ctx.build_args.content.clone();
    let output_folder = ctx.build_args.output.clone();

    let web_state = Arc::new(RwLock::new(
        WebState::builder()
            .content_path(content_folder)
            .output_path(output_folder)
            .build(),
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
    let binary_dir = std::env::current_dir()?;

    watcher
        .watch(
            watcher_ctx.build_args.cli_args.content.clone(),
            move |event: hotwatch::Event| {
                match handle_watch()
                    .ctx(&mut watcher_ctx)
                    .event(event)
                    .binary_dir(binary_dir.clone())
                    .call()
                {
                    Ok(_) => {}
                    Err(e) => {
                        tracing::error!(
                            "failed to executed the watcher function, Error: {:#?}",
                            e.to_string()
                        );
                        std::process::exit(1);
                    }
                };
                reloader.reload();
            },
        )
        .wrap_err("failed to watch for changes using hotwatch")?;

    let server_port = ctx.config.port.clone();
    let listener = tokio::net::TcpListener::bind(ADDRESS.to_string() + ":" + &server_port)
        .await
        .wrap_err("failed to bind TCP Listener to address")?;

    if ctx.config.logging == true {
        tracing::info!("running on {}:{}", ADDRESS, server_port);
    }

    axum::serve(listener, router)
        .await
        .wrap_err("Failed to start the serrver listener")?;

    return Ok(());
}

#[bon::builder]
pub fn handle_watch(
    event: hotwatch::Event,
    binary_dir: path::PathBuf,
    ctx: &mut context::Context,
) -> eyre::Result<(), error::Error> {
    match event.kind {
        hotwatch::EventKind::Modify(hotwatch::notify::event::ModifyKind::Data(_))
        | hotwatch::EventKind::Create(_) => {
            let event_path = event.paths[0].to_string_lossy().to_string();
            let content_path = std::fs::canonicalize(
                binary_dir.join(path::PathBuf::from(ctx.build_args.cli_args.content.clone())),
            )?
            .to_string_lossy()
            .to_string();

            // this is the relative path of the modified file with respect to the content folder
            let curr_build_rel_path =
                pathdiff::diff_paths(event_path.clone(), content_path.clone())
                    .ok_or_else(|| {
                        eyre::eyre!(
                            "failed to get the difference between paths: {} and {}",
                            event_path.clone(),
                            content_path.clone()
                        )
                    })?
                    .to_string_lossy()
                    .to_string();

            ctx.is_rebuild = true;
            ctx.build_path = ctx.build_args.content.join(curr_build_rel_path)?;

            // skip files hidden files and any files other than markdown files
            if ctx.build_path.is_file()?
                && (!utils::is_markdown_file().file_path(&ctx.build_path).call()
                    || utils::is_hidden_file().file_path(&ctx.build_path).call())
            {
                return Ok(());
            }

            ctx.file_tree = helpers::file_tree::map_folder()
                .input_path(ctx.build_args.content.clone())
                .call()?;

            let parsed_files = processors::parse().ctx(&ctx).call()?;
            processors::emit()
                .ctx(&ctx)
                .parsed_files(&parsed_files)
                .call()?;
        }
        hotwatch::EventKind::Remove(_) => {}
        _ => {}
    }
    return Ok(());
}
