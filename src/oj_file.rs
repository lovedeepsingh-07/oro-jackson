use bon;
use serde_yaml;
use toml;

#[derive(Debug, Clone, bon::Builder)]
pub struct OjFile {
    pub frontmatter: OjFrontmatter,
    pub input_path: vfs::VfsPath,
    pub output_path: vfs::VfsPath,
    pub content: String,
}

#[derive(Debug, Clone)]
pub enum OjFrontmatter {
    Toml(toml::Value),
    Yaml(serde_yaml::Value),
}
