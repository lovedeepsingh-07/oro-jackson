use clap::Parser;
use color_eyre::{self, eyre};
use oro_jackson::{cli, context, error, processors, server};
use std::{fs, path};
use tokio;
use vfs;

#[derive(rust_embed::RustEmbed, Clone, Debug)]
#[folder = "default"]
pub struct DefaultFiles;

pub fn get_embedded_default_file(filepath: String) -> eyre::Result<String, error::Error> {
    let file = DefaultFiles::get(filepath.as_str()).ok_or_else(|| {
        error::Error::NotFound("no such embedded default file or directory".to_string())
    })?;
    let content = String::from_utf8(file.data.to_vec())?;
    return Ok(content);
}

#[tokio::main]
async fn main() -> eyre::Result<(), error::Error> {
    color_eyre::install()?;
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_max_level(tracing::Level::INFO)
        .init();

    let cli_args = cli::CLIArgs::parse();

    match &cli_args.sub_commands {
        cli::SubCommands::Create => {
            let curr_dir = std::env::current_dir()?;
            let project_fs: vfs::PhysicalFS = vfs::PhysicalFS::new(curr_dir);
            let project_root: vfs::VfsPath = project_fs.into();

            for item in DefaultFiles::iter() {
                let item_path = project_root.join(item.to_string())?;

                let item_content = get_embedded_default_file(item.to_string())?;

                let parent_folder = item_path.parent();
                parent_folder.create_dir_all()?;

                let mut f = item_path.create_file()?;
                f.write_all(item_content.as_bytes())?;

                tracing::info!("Successfully created {:#?}", item_path.as_str());
            }
        }
        cli::SubCommands::Build(cli_data) => {
            let output_path_fs: vfs::PhysicalFS = vfs::PhysicalFS::new(&cli_data.output);
            let output_path_root: vfs::VfsPath = output_path_fs.into();

            let content_path_fs: vfs::PhysicalFS = vfs::PhysicalFS::new(&cli_data.content);
            let content_path_root: vfs::VfsPath = content_path_fs.into();

            // if the build directory already exists upon build, we must remove it
            if output_path_root.exists()? {
                output_path_root.remove_dir_all()?;
            }
            // then create an empty build directory for a clean build
            fs::create_dir_all(&cli_data.output)?;

            let config_path: path::PathBuf = path::PathBuf::from(&cli_data.config);
            let config_file_content: String = fs::read_to_string(config_path)?;

            let theme_path: path::PathBuf = path::PathBuf::from(&cli_data.theme);
            let theme_file_content: String = fs::read_to_string(theme_path)?;

            let build_args = context::BuildArgs::builder()
                .content(content_path_root)
                .output(output_path_root)
                .serve(cli_data.serve)
                .cli_args(cli_data.clone())
                .build();

            let ctx = context::Context::builder()
                .build_args(build_args)
                .config_file_content(&config_file_content)
                .theme_file_content(&theme_file_content)
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
