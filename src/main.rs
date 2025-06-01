use clap::Parser;
use color_eyre::{
    self,
    eyre::{self},
};
use oro_jackson::{cli, context, error, processors, server};
use std::{fs, path};
use tokio;

#[tokio::main]
async fn main() -> eyre::Result<(), error::Error> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_max_level(tracing::Level::INFO)
        .init();

    let cli_args = cli::CLIArgs::parse();

    match &cli_args.sub_commands {
        cli::SubCommands::Build(cli_data) => {
            // if the build directory already exists upon build, we must remove it
            if path::Path::new(&cli_data.output).exists() {
                fs::remove_dir_all(&cli_data.output)?;
            }

            let config_file_contents = fs::read_to_string(cli_data.config.clone())?;

            let ctx = context::Context::builder()
                .config_file_content(&config_file_contents)
                .build_args(context::BuildArgs::from(cli_data.clone()))
                .build()?;

            let parsed_files = processors::parse().ctx(&ctx).call()?;
            processors::emit()
                .ctx(&ctx)
                .parsed_files(&parsed_files)
                .call()?;

            if cli_data.serve == true {
                server::serve().ctx(&ctx).call().await?;
            }
        }
    };
    return Ok(());
}
