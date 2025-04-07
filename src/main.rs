// imports
use clap::Parser;
use oro_jackson::{cli, content, server};
use tokio;

#[tokio::main]
async fn main() {
    let args = cli::CLIArgs::parse();

    match &args.sub_commands {
        cli::SubCommands::Serve(server_data) => {
            match server::serve()
                .server_data(server_data.clone())
                .call()
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Failed to start the server, Error: {0}", e.to_string());
                    std::process::exit(1);
                }
            }
        }
        cli::SubCommands::Build(data) => {
            match content::build_content()
                .content_folder_path(data.content.clone().as_str())
                .output_folder_path(data.output.clone().as_str())
                .input_path_string(data.content.clone().as_str())
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
