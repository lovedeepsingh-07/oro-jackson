// imports
use clap::{Args, Parser, Subcommand};

// modules
#[cfg(test)]
mod tests;

// constants
pub const ADDRESS: std::net::Ipv4Addr = std::net::Ipv4Addr::new(0, 0, 0, 0);
pub const PORT: u16 = 8080;

// ----- `CLIArgs` struct
#[derive(Parser, Debug, Clone)]
#[command(about,long_about=None)]
#[command(next_line_help = true)]
pub struct CLIArgs {
    #[command(subcommand)]
    pub sub_commands: SubCommands,
}

// ----- `SubCommands` for the CLIArgs
#[derive(Subcommand, Debug, Clone)]
pub enum SubCommands {
    Serve(Serve),
    Build(Build),
}

// ----- `Serve` subcommand
#[derive(Args, Debug, Clone)]
#[command(about = "Serve the content")]
pub struct Serve {
    #[arg(long, help = "path location of your content folder")]
    pub content: String,
    #[arg(long, help = "path location of output folder")]
    pub output: String,
}

// ----- `Build` subcommand
#[derive(Args, Debug, Clone)]
#[command(about = "Build the content")]
pub struct Build {
    #[arg(long, help = "path location of your content folder")]
    pub content: String,
    #[arg(long, help = "path location of output folder")]
    pub output: String,
}
