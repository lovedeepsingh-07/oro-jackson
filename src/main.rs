// imports
use axum;
use clap::Parser;
use oro_jackson::{self, cli, handlers, server};
use std::sync::{Arc, RwLock};
use tokio;

#[tokio::main]
async fn main() {
    // get CLI arguments
    let args = cli::CLIArgs::parse();

    // match sub-commands
    match &args.sub_commands {
        // `serve` subcommand
        cli::SubCommands::Serve(data) => {
            // web state for the main router
            let web_state = Arc::new(RwLock::new(
                match server::WebState::builder()
                    .content_path(data.content.clone())
                    .build()
                {
                    Ok(safe_web_state) => safe_web_state,
                    Err(e) => {
                        eprintln!("Failed to create web state object, Error: {:#?}", e);
                        std::process::exit(1);
                    }
                },
            ));

            // main router
            let router: axum::Router = axum::Router::new()
                .route("/", axum::routing::get(handlers::home_route))
                .with_state(web_state);

            // bind a `TcpListener` to an address and port
            let listener = match tokio::net::TcpListener::bind(
                oro_jackson::ADDRESS.to_string() + ":" + oro_jackson::PORT.to_string().as_str(),
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
            println!("running on {}:{}", oro_jackson::ADDRESS, oro_jackson::PORT);

            // actually start the server listener
            match axum::serve(listener, router).await {
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
        // `build` subcommand
        cli::SubCommands::Build(data) => {
            println!("{:#?}", data);
        }
    }
}
