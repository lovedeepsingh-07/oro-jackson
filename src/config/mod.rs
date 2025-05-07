use serde;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub show_folder_page_children: bool,
    pub theme: Theme,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Theme {
    pub light: Light,
    pub dark: Dark,
    pub radius: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Light {
    pub background: String,
    pub foreground: String,
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub neutral: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Dark {
    pub background: String,
    pub foreground: String,
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub neutral: String,
}
