use crate::plugins;
use serde;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub settings: Settings,
    pub server: Server,
    pub plugins: Plugins,
    pub theme: Theme,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    pub logging: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Server {
    pub port: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Plugins {
    pub transformers: Transformers,
    pub emitters: Emitters,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Transformers {
    pub markdown: plugins::transformers::markdown::MarkdownTransformerOptions,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Emitters {
    pub static_assets: plugins::emitters::static_assets::StaticAssetsEmitterOptions,
    pub file_page: plugins::emitters::file_page::FilePageEmitterOptions,
    pub folder_page: plugins::emitters::folder_page::FolderPageEmitterOptions,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Theme {
    pub light: Light,
    pub dark: Dark,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Light {
    pub background: String,
    pub foreground: String,
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub neutral: String,
    pub radius: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Dark {
    pub background: String,
    pub foreground: String,
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub neutral: String,
    pub radius: String,
}
