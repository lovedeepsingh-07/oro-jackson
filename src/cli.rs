use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(about,long_about=None)]
#[command(next_line_help = true)]
pub struct CLIArgs {
    #[command(subcommand)]
    pub sub_commands: SubCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum SubCommands {
    Create,
    Build(Build),
}

#[derive(Args, Debug, Clone)]
#[command(about = "Build the content")]
pub struct Build {
    #[arg(long, help = "path location of your config.toml file")]
    pub config: String,
    #[arg(long, help = "path location of your theme.css file")]
    pub theme: String,
    #[arg(long, help = "path location of your content folder")]
    pub content: String,
    #[arg(long, help = "path location of output folder")]
    pub output: String,
    #[arg(long, help = "serve the content and watch for changes")]
    pub serve: bool,
}
