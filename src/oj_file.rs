use bon;
use serde_yaml;
use toml;

#[derive(Debug, Clone, bon::Builder)]
pub struct OjFile {
    pub frontmatter: OjFrontmatter,
    pub input_path: String,
    pub abs_input_path: String,
    pub output_path: String,
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
            input_path: String::new(),
            abs_input_path: String::new(),
            output_path: String::new(),
            content: String::new(),
        };
    }
}
