use clap::Parser;
use color_eyre::{self, eyre};
use oro_jackson::{cli, context, error, processors, server};
use std::{fs, path};
use tokio;
use vfs;

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
            let output_path_fs: vfs::PhysicalFS = vfs::PhysicalFS::new(&cli_data.output);
            let output_path_root: vfs::VfsPath = output_path_fs.into();

            let content_path_fs: vfs::PhysicalFS = vfs::PhysicalFS::new(&cli_data.content);
            let content_path_root: vfs::VfsPath = content_path_fs.into();

            let config_path: path::PathBuf = path::PathBuf::from(&cli_data.config);

            // if the build directory already exists upon build, we must remove it
            if output_path_root.exists()? {
                output_path_root.remove_dir_all()?;
            }
            fs::create_dir_all(&cli_data.output)?;

            let config_file_content: String = fs::read_to_string(config_path)?;

            let build_args = context::BuildArgs::builder()
                .content(content_path_root)
                .output(output_path_root)
                .serve(cli_data.serve)
                .cli_args(cli_data.clone())
                .build();

            let ctx = context::Context::builder()
                .build_args(build_args)
                .config_file_content(&config_file_content)
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
