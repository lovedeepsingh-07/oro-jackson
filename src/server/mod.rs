use crate::{context, error};
use axum;
use bon;
use color_eyre::eyre::{self, WrapErr};
use std::sync::Arc;
use tokio::{self, sync::RwLock};
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

    let router: axum::Router = axum::Router::new()
        .route("/", axum::routing::get(handlers::index_route))
        .route("/*filepath", axum::routing::get(handlers::main_route))
        .route(
            "/favicon.ico",
            axum::routing::get(|| async { axum::http::StatusCode::NO_CONTENT }),
        )
        .with_state(web_state);

    // TODO: reimplement live-realoding using `vfs`

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
