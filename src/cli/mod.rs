use clap::{Args, Parser, Subcommand};

#[cfg(test)]
mod tests;

#[derive(Parser, Debug, Clone)]
#[command(about,long_about=None)]
#[command(next_line_help = true)]
pub struct CLIArgs {
    #[command(subcommand)]
    pub sub_commands: SubCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum SubCommands {
    Serve(Serve),
    Build(Build),
}

#[derive(Args, Debug, Clone)]
#[command(about = "Serve the content")]
pub struct Serve {
    #[arg(long, help = "path location of your content folder")]
    pub content: String,
    #[arg(long, help = "path location of output folder")]
    pub output: String,
}

#[derive(Args, Debug, Clone)]
#[command(about = "Build the content")]
pub struct Build {
    #[arg(long, help = "path location of your content folder")]
    pub content: String,
    #[arg(long, help = "path location of output folder")]
    pub output: String,
}
