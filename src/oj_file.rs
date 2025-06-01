use bon;
use serde_yaml;
use std::path;
use toml;

#[derive(Debug, Clone, bon::Builder)]
pub struct OjFile {
    pub frontmatter: OjFrontmatter,
    pub input_path: path::PathBuf,
    pub abs_input_path: path::PathBuf,
    pub output_path: path::PathBuf,
    pub content: String,
}

#[derive(Debug, Clone)]
pub enum OjFrontmatter {
    Toml(toml::Value),
    Yaml(serde_yaml::Value),
}

impl Default for OjFile {
    fn default() -> Self {
        return OjFile {
            frontmatter: OjFrontmatter::Yaml(serde_yaml::Value::Null),
            input_path: path::PathBuf::new(),
            abs_input_path: path::PathBuf::new(),
            output_path: path::PathBuf::new(),
            content: String::new(),
        };
    }
}
