// modules
mod core;
mod handlers;
mod templates;
mod utils;

// imports
use axum;
use axum_embed;
use clap::Parser;
use std::sync::{Arc, RwLock};
use tokio;

#[tokio::main]
async fn main() {
    // get CLI arguments
    let args = core::app::CLIArgs::parse();

    // match sub-commands
    match &args.sub_commands {
        core::app::SubCommands::Serve(data) => {
            // application state for the main router
            let app_state = Arc::new(RwLock::new(
                match core::app::Application::new(data.clone()) {
                    Ok(safe_app) => safe_app,
                    Err(e) => {
                        eprintln!("Failed to create application state object, Error: {:#?}", e);
                        std::process::exit(1);
                    }
                },
            ));

            // main application router
            let app_router: axum::Router = axum::Router::new()
                .nest_service(
                    // static files route
                    "/static",
                    axum_embed::ServeEmbed::<utils::StaticAssets>::new(),
                )
                .route("/", axum::routing::get(handlers::home_page))
                .route(
                    "/vault/*note_path",
                    axum::routing::get(handlers::vault_page),
                )
                .with_state(app_state);

            // bind a `TcpListener` to an address and port
            let listener = match tokio::net::TcpListener::bind(
                core::ADDRESS.to_string() + ":" + core::PORT.to_string().as_str(),
            )
            .await
            {
                Ok(safe_listener) => safe_listener,
                Err(e) => {
                    eprintln!("Failed to bind TcpListener to address, Error: {:#?}", e);
                    std::process::exit(1);
                }
            };

            // ----- announce the application startup -----
            println!("running on {}:{}", core::ADDRESS, core::PORT);

            // actually start the server listener
            match axum::serve(listener, app_router).await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!(
                        "Failed to start the server listener for the application, Error: {:#?}",
                        e
                    );
                    std::process::exit(1);
                }
            };
        }
    }
}
