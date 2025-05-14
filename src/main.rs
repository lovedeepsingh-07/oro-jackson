use clap::Parser;
use color_eyre::{
    self,
    eyre::{self, WrapErr},
};
use oro_jackson::{cli, config, error, processors};
use std::fs;
use tokio;

#[tokio::main]
async fn main() -> eyre::Result<(), error::Error> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_max_level(tracing::Level::INFO)
        .init();

    let args = cli::CLIArgs::parse();

    match &args.sub_commands {
        cli::SubCommands::Build(cli_data) => {
            let config_file_path_canon = fs::canonicalize(cli_data.config.clone())?;
            let config_file_contents = fs::read_to_string(&config_file_path_canon)?;
            let app_config: config::Config = toml::from_str(&config_file_contents)?;

            processors::parse::parse()
                .content_folder_path(cli_data.content.clone().as_str())
                .output_folder_path(cli_data.output.clone().as_str())
                .input_path_string(cli_data.content.clone().as_str())
                .call()
                .wrap_err("failed to build content")?;

            processors::parse::build_index_files()
                .output_folder_path(cli_data.output.clone())
                .call()
                .wrap_err("failed to build index files for folders")?;

            processors::parse::build_static_assets()
                .output_folder_path(cli_data.output.clone().as_str())
                .app_config(app_config)
                .call()
                .wrap_err("failed to build static assets")?;
        }
    };
    return Ok(());
}
