// imports
use clap::{Args, Parser, Subcommand};

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
}

// ----- `Serve` subcommand
#[derive(Args, Debug, Clone)]
#[command(about = "Serve the application")]
pub struct Serve {}

// ----- `Application` object
#[derive(Debug, Clone)]
pub struct Application {}
impl Application {
    pub fn new(data: Serve) -> Result<Application, String> {
        println!("found some data: {:#?}", data);
        return Ok(Application {});
    }
}
