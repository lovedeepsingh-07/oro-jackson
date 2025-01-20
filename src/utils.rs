// imports
use rust_embed;
use std::{fs, path::Path};

// ----- `StaticAssets` object
#[derive(rust_embed::RustEmbed, Clone)]
#[folder = "static/"]
pub struct StaticAssets;

// implement a way to map the entire obsidian vault into a hashmap kind of thing maybe
#[allow(dead_code)]
pub fn map_vault(vault_path_string: String) {
    let vault_path = Path::new(&vault_path_string);
    if vault_path.is_dir() {
        match fs::read_dir(vault_path) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        println!("{:#?}", entry.path().to_string_lossy().to_string());
                    }
                }
            }
            Err(e) => println!("Error reading directory: {}", e),
        }
    }
}
