// imports
use crate::utils;
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
pub struct Serve {
    #[arg(long, help = "path location of your obsidian vault")]
    pub vault: String,
}

// ----- `Application` object
#[derive(Debug, Clone)]
pub struct Application {
    pub vault_map: Vec<utils::VaultObject>,
}
impl Application {
    pub fn new(data: Serve) -> Result<Application, String> {
        let vault_map = utils::map_vault_object(data.vault).unwrap();
        return Ok(Application { vault_map });
    }
}
