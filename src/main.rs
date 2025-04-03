// imports
use axum;
use clap::Parser;
use hotwatch;
use oro_jackson::{self, cli, content, server};
use std::sync::{Arc, RwLock};
use tokio;
use tower_livereload;

#[tokio::main]
async fn main() {
    // get CLI arguments
    let args = cli::CLIArgs::parse();

    // match sub-commands
    match &args.sub_commands {
        // `serve` subcommand
        cli::SubCommands::Serve(data) => {
            let content_folder = data.content.clone();
            let output_folder = data.output.clone();

            // build content
            match content::build_content()
                .content_path(content_folder.clone().as_str())
                .output_path(output_folder.clone().as_str())
                .call()
            {
                Ok(_) => {}
                Err(e) => {
                    eprintln!(
                        "Failed to map the content folder, Error: {:#?}",
                        e.to_string()
                    );
                    std::process::exit(1);
                }
            };

            // web state for the main router
            let web_state = Arc::new(RwLock::new(
                match server::WebState::builder()
                    .content_path(content_folder.clone())
                    .output_path(output_folder.clone())
                    .build()
                {
                    Ok(safe_web_state) => safe_web_state,
                    Err(e) => {
                        eprintln!("Failed to create web state object, Error: {:#?}", e);
                        std::process::exit(1);
                    }
                },
            ));

            // live-reload setup
            let live_reload_layer = tower_livereload::LiveReloadLayer::new();
            let reloader = live_reload_layer.reloader();

            // main router
            let router: axum::Router = axum::Router::new()
                .route("/", axum::routing::get(oro_jackson::handlers::index_route))
                .route(
                    "/*filepath",
                    axum::routing::get(oro_jackson::handlers::main_route),
                )
                .with_state(web_state)
                .layer(live_reload_layer);

            // watching for file changes in `content` directory
            let mut watcher =
                hotwatch::Hotwatch::new_with_custom_delay(std::time::Duration::from_millis(100))
                    .unwrap();
            watcher
                .watch(content_folder.clone(), move |_event: hotwatch::Event| {
                    // build content
                    match content::build_content()
                        .content_path(content_folder.clone().as_str())
                        .output_path(output_folder.clone().as_str())
                        .call()
                    {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!(
                                "Failed to map the content folder, Error: {:#?}",
                                e.to_string()
                            );
                            std::process::exit(1);
                        }
                    };
                    // reload server
                    reloader.reload();
                })
                .unwrap();

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
            match content::build_content()
                .content_path(&data.content)
                .output_path(&data.output)
                .call()
            {
                Ok(_) => {}
                Err(e) => {
                    eprintln!(
                        "Failed to map the content folder, Error: {:#?}",
                        e.to_string()
                    );
                    std::process::exit(1);
                }
            };
        }
    }
}
