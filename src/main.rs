// modules
mod core;

// imports
use clap::Parser;

fn main() {
    // get CLI arguments
    let args = core::app::CLIArgs::parse();

    // match sub-commands
    match &args.sub_commands {
        core::app::SubCommands::Serve(data) => {
            let app = match core::app::Application::new() {
                Ok(safe_app) => safe_app,
                Err(e) => {
                    eprintln!("Failed to create application state object, Error: {:#?}", e);
                    std::process::exit(1);
                }
            };
            app.run(data.clone());
        }
    }
}
