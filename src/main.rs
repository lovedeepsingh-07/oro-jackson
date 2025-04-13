use clap::Parser;
use color_eyre::{
    self,
    eyre::{self, WrapErr},
};
use oro_jackson::{cli, content, error, server};
use tokio;

#[tokio::main]
async fn main() -> eyre::Result<(), error::Error> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let args = cli::CLIArgs::parse();

    match &args.sub_commands {
        cli::SubCommands::Serve(server_data) => {
            server::serve()
                .server_data(server_data.clone())
                .call()
                .await
                .wrap_err("failed to start the server")?;
        }
        cli::SubCommands::Build(data) => {
            content::build_content()
                .content_folder_path(data.content.clone().as_str())
                .output_folder_path(data.output.clone().as_str())
                .input_path_string(data.content.clone().as_str())
                .call()
                .wrap_err("failed to build content")?;

            content::build_index_files()
                .output_folder_path(data.output.clone())
                .call()
                .wrap_err("failed to build index files for folders")?;

            content::build_static_assets()
                .output_folder_path(data.output.clone())
                .call()
                .wrap_err("failed to build static assets")?;
        }
    };
    return Ok(());
}
