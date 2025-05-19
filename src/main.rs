use clap::Parser;
use color_eyre::{
    self,
    eyre::{self},
};
use oro_jackson::{cli, config, context, error, processors};
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

            let config_file_path_canon = fs::canonicalize(cli_data.config.clone())?;
            let config_file_contents = fs::read_to_string(&config_file_path_canon)?;
            let app_config: config::Config = toml::from_str(&config_file_contents)?;

            let mut ctx = context::Context::builder()
                .app_config(app_config)
                .build_args(cli_data.clone())
                .build()?;

            let parsed_files = processors::parse().ctx(&mut ctx).call()?;
            processors::emit()
                .ctx(&mut ctx)
                .parsed_files(&parsed_files)
                .call()?;
        }
    };
    return Ok(());
}
