use crate::plugins;
use serde;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub title: String,
    pub logging: bool,
    pub port: String,
    pub file_explorer: bool,
    pub plugins: Plugins,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Plugins {
    pub transformers: Transformers,
    pub emitters: Emitters,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Transformers {
    pub frontmatter: plugins::transformers::frontmatter::FrontmatterTransformerOptions,
    pub markdown: plugins::transformers::markdown::MarkdownTransformerOptions,
    pub sanitize: plugins::transformers::sanitize::SanitizeTransformerOptions,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Emitters {
    pub static_assets: plugins::emitters::static_assets::StaticAssetsEmitterOptions,
    pub file_page: plugins::emitters::file_page::FilePageEmitterOptions,
    pub folder_page: plugins::emitters::folder_page::FolderPageEmitterOptions,
}
