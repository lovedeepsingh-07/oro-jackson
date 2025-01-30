// imports
use rust_embed;
use std::{fs, path};
use urlencoding;

// ----- `StaticAssets` object
#[derive(rust_embed::RustEmbed, Clone)]
#[folder = "static/"]
pub struct StaticAssets;

// get files contents from embedded files i.e `static` directory
pub fn get_embedded_file(filepath: String) -> Option<Result<String, String>> {
    match StaticAssets::get(filepath.as_str()) {
        Some(file_content) => {
            return Some(match String::from_utf8(file_content.data.to_vec()) {
                Ok(safe_value) => Ok(safe_value),
                Err(e) => Err(e.to_string()),
            });
        }
        None => {
            return None;
        }
    }
}

// implement a way to map the entire obsidian vault into a hashmap kind of thing maybe
#[allow(dead_code)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum VaultObject {
    File(VaultObjectFile),
    Folder(VaultObjectFolder),
}
#[allow(dead_code)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VaultObjectFile {
    pub name: String,
    pub slug: String,
    pub path: String,
}
#[allow(dead_code)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VaultObjectFolder {
    pub name: String,
    pub slug: String,
    pub path: String,
    pub children: Vec<VaultObject>,
}
pub fn map_vault_object(data: String) -> Result<Vec<VaultObject>, String> {
    let vault_path_string = format!("{}", data);
    let vault_path = path::Path::new(&vault_path_string);
    let ignores = Vec::from([".git", ".obsidian"]);
    let mut vault_vec = Vec::new();

    if vault_path.is_dir() {
        match fs::read_dir(vault_path) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let entry_path = entry.path();
                        let entry_name = entry.file_name().to_string_lossy().to_string();
                        if !ignores.contains(&entry_name.as_str()) {
                            if entry_path.is_dir() {
                                vault_vec.push(VaultObject::Folder(VaultObjectFolder {
                                    name: entry_name.clone(),
                                    slug: name_to_slug(entry_path.to_string_lossy().to_string()),
                                    path: entry_path.to_string_lossy().to_string(),
                                    children: map_vault_object(
                                        entry_path.to_string_lossy().to_string(),
                                    )
                                    .unwrap(),
                                }));
                            } else {
                                vault_vec.push(VaultObject::File(VaultObjectFile {
                                    name: entry_name.clone(),
                                    slug: name_to_slug(entry_path.to_string_lossy().to_string()),
                                    path: entry_path.to_string_lossy().to_string(),
                                }));
                            }
                        }
                    }
                }
            }
            Err(e) => eprintln!("Error reading directory: {}", e),
        }
    }
    return Ok(vault_vec);
}

pub fn name_to_slug(input: String) -> String {
    return urlencoding::encode(&input).to_string();
}
