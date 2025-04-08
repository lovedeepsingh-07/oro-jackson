use crate::{cli, content, error, handlers};
use axum;
use bon;
use hotwatch;
use std::sync::{Arc, RwLock};
use tower_livereload;

#[cfg(test)]
mod tests;

// constants
pub const ADDRESS: std::net::Ipv4Addr = std::net::Ipv4Addr::new(0, 0, 0, 0);
pub const PORT: u16 = 8080;

#[derive(Debug, Clone)]
pub struct WebState {
    pub content_path: String,
    pub output_path: String,
}
#[bon::bon]
impl WebState {
    #[builder]
    pub fn new(content_path: String, output_path: String) -> Result<WebState, error::ServerError> {
        return Ok(WebState {
            content_path,
            output_path,
        });
    }
}

#[bon::builder]
pub async fn serve(server_data: cli::Serve) -> Result<(), error::ServerError> {
    let content_folder = server_data.content.clone();
    let output_folder = server_data.output.clone();

    match content::build_content()
        .content_folder_path(content_folder.clone().as_str())
        .output_folder_path(output_folder.clone().as_str())
        .input_path_string(content_folder.clone().as_str())
        .call()
    {
        Ok(_) => {}
        Err(e) => return Err(error::ServerError::ContentBuildError(e.to_string())),
    };

    match content::build_static_assets()
        .output_folder_path(server_data.output.clone())
        .call()
    {
        Ok(_) => {}
        Err(e) => return Err(error::ServerError::StaticAssetsBuildError(e.to_string())),
    };

    let web_state = Arc::new(RwLock::new(
        match WebState::builder()
            .content_path(content_folder.clone())
            .output_path(output_folder.clone())
            .build()
        {
            Ok(safe_web_state) => safe_web_state,
            Err(e) => return Err(error::ServerError::WebStateError(e.to_string())),
        },
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
        hotwatch::Hotwatch::new_with_custom_delay(std::time::Duration::from_millis(100)).unwrap();
    watcher
        .watch(
            content_folder.clone(),
            move |event: hotwatch::Event| match event.kind {
                hotwatch::EventKind::Modify(hotwatch::notify::event::ModifyKind::Data(_))
                | hotwatch::EventKind::Create(_) => {
                    match content::build_content()
                        .content_folder_path(content_folder.clone().as_str())
                        .output_folder_path(output_folder.clone().as_str())
                        .input_path_string(event.paths[0].to_string_lossy().to_string().as_str())
                        .call()
                    {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("Failed to build content , Error: {:#?}", e.to_string());
                            std::process::exit(1);
                        }
                    };
                    reloader.reload();
                }
                hotwatch::EventKind::Remove(_) => {}
                _ => {}
            },
        )
        .unwrap();

    let listener =
        match tokio::net::TcpListener::bind(ADDRESS.to_string() + ":" + PORT.to_string().as_str())
            .await
        {
            Ok(safe_listener) => safe_listener,
            Err(e) => return Err(error::ServerError::TCPListenerBindError(e.to_string())),
        };

    println!("running on {}:{}", ADDRESS, PORT);

    match axum::serve(listener, router).await {
        Ok(_) => {}
        Err(e) => return Err(error::ServerError::ServerListenerStartError(e.to_string())),
    };

    return Ok(());
}
