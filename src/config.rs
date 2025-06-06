use crate::plugins;
use serde;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub title: String,
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Theme {
    pub light: OjTheme,
    pub dark: OjTheme,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OjTheme {
    pub color_base_100: String,
    pub color_base_200: String,
    pub color_base_300: String,
    pub color_base_content: String,
    pub color_primary: String,
    pub color_primary_content: String,
    pub color_secondary: String,
    pub color_secondary_content: String,
    pub color_accent: String,
    pub color_accent_content: String,
    pub color_neutral: String,
    pub color_neutral_content: String,
    pub color_info: String,
    pub color_info_content: String,
    pub color_success: String,
    pub color_success_content: String,
    pub color_warning: String,
    pub color_warning_content: String,
    pub color_error: String,
    pub color_error_content: String,
    pub radius_selector: String,
    pub radius_field: String,
    pub radius_box: String,
    pub size_selector: String,
    pub size_field: String,
    pub border: String,
    pub depth: String,
    pub noise: String,
}

impl Theme {
    pub fn to_css(&self) -> String {
        let mut css = String::from(":root {\n");

        css.push_str(&format!(
            "\t--light-color-base-100: {};\n",
            self.light.color_base_100
        ));
        css.push_str(&format!(
            "\t--light-color-base-200: {};\n",
            self.light.color_base_200
        ));
        css.push_str(&format!(
            "\t--light-color-base-300: {};\n",
            self.light.color_base_300
        ));
        css.push_str(&format!(
            "\t--light-color-base-content: {};\n",
            self.light.color_base_content
        ));
        css.push_str(&format!(
            "\t--light-color-primary: {};\n",
            self.light.color_primary
        ));
        css.push_str(&format!(
            "\t--light-color-primary-content: {};\n",
            self.light.color_primary_content
        ));
        css.push_str(&format!(
            "\t--light-color-secondary: {};\n",
            self.light.color_secondary
        ));
        css.push_str(&format!(
            "\t--light-color-secondary-content: {};\n",
            self.light.color_secondary_content
        ));
        css.push_str(&format!(
            "\t--light-color-accent: {};\n",
            self.light.color_accent
        ));
        css.push_str(&format!(
            "\t--light-color-accent-content: {};\n",
            self.light.color_accent_content
        ));
        css.push_str(&format!(
            "\t--light-color-neutral: {};\n",
            self.light.color_neutral
        ));
        css.push_str(&format!(
            "\t--light-color-neutral-content: {};\n",
            self.light.color_neutral_content
        ));
        css.push_str(&format!(
            "\t--light-color-info: {};\n",
            self.light.color_info
        ));
        css.push_str(&format!(
            "\t--light-color-info-content: {};\n",
            self.light.color_info_content
        ));
        css.push_str(&format!(
            "\t--light-color-success: {};\n",
            self.light.color_success
        ));
        css.push_str(&format!(
            "\t--light-color-success-content: {};\n",
            self.light.color_success_content
        ));
        css.push_str(&format!(
            "\t--light-color-warning: {};\n",
            self.light.color_warning
        ));
        css.push_str(&format!(
            "\t--light-color-warning-content: {};\n",
            self.light.color_warning_content
        ));
        css.push_str(&format!(
            "\t--light-color-error: {};\n",
            self.light.color_error
        ));
        css.push_str(&format!(
            "\t--light-color-error-content: {};\n",
            self.light.color_error_content
        ));
        css.push_str(&format!(
            "\t--light-radius-selector: {};\n",
            self.light.radius_selector
        ));
        css.push_str(&format!(
            "\t--light-radius-field: {};\n",
            self.light.radius_field
        ));
        css.push_str(&format!(
            "\t--light-radius-box: {};\n",
            self.light.radius_box
        ));
        css.push_str(&format!(
            "\t--light-size-selector: {};\n",
            self.light.size_selector
        ));
        css.push_str(&format!(
            "\t--light-size-field: {};\n",
            self.light.size_field
        ));
        css.push_str(&format!("\t--light-border: {};\n", self.light.border));
        css.push_str(&format!("\t--light-depth: {};\n", self.light.depth));
        css.push_str(&format!("\t--light-noise: {};\n", self.light.noise));

        css.push_str(&format!(
            "\t--dark-color-base-100: {};\n",
            self.dark.color_base_100
        ));
        css.push_str(&format!(
            "\t--dark-color-base-200: {};\n",
            self.dark.color_base_200
        ));
        css.push_str(&format!(
            "\t--dark-color-base-300: {};\n",
            self.dark.color_base_300
        ));
        css.push_str(&format!(
            "\t--dark-color-base-content: {};\n",
            self.dark.color_base_content
        ));
        css.push_str(&format!(
            "\t--dark-color-primary: {};\n",
            self.dark.color_primary
        ));
        css.push_str(&format!(
            "\t--dark-color-primary-content: {};\n",
            self.dark.color_primary_content
        ));
        css.push_str(&format!(
            "\t--dark-color-secondary: {};\n",
            self.dark.color_secondary
        ));
        css.push_str(&format!(
            "\t--dark-color-secondary-content: {};\n",
            self.dark.color_secondary_content
        ));
        css.push_str(&format!(
            "\t--dark-color-accent: {};\n",
            self.dark.color_accent
        ));
        css.push_str(&format!(
            "\t--dark-color-accent-content: {};\n",
            self.dark.color_accent_content
        ));
        css.push_str(&format!(
            "\t--dark-color-neutral: {};\n",
            self.dark.color_neutral
        ));
        css.push_str(&format!(
            "\t--dark-color-neutral-content: {};\n",
            self.dark.color_neutral_content
        ));
        css.push_str(&format!("\t--dark-color-info: {};\n", self.dark.color_info));
        css.push_str(&format!(
            "\t--dark-color-info-content: {};\n",
            self.dark.color_info_content
        ));
        css.push_str(&format!(
            "\t--dark-color-success: {};\n",
            self.dark.color_success
        ));
        css.push_str(&format!(
            "\t--dark-color-success-content: {};\n",
            self.dark.color_success_content
        ));
        css.push_str(&format!(
            "\t--dark-color-warning: {};\n",
            self.dark.color_warning
        ));
        css.push_str(&format!(
            "\t--dark-color-warning-content: {};\n",
            self.dark.color_warning_content
        ));
        css.push_str(&format!(
            "\t--dark-color-error: {};\n",
            self.dark.color_error
        ));
        css.push_str(&format!(
            "\t--dark-color-error-content: {};\n",
            self.dark.color_error_content
        ));
        css.push_str(&format!(
            "\t--dark-radius-selector: {};\n",
            self.dark.radius_selector
        ));
        css.push_str(&format!(
            "\t--dark-radius-field: {};\n",
            self.dark.radius_field
        ));
        css.push_str(&format!("\t--dark-radius-box: {};\n", self.dark.radius_box));
        css.push_str(&format!(
            "\t--dark-size-selector: {};\n",
            self.dark.size_selector
        ));
        css.push_str(&format!("\t--dark-size-field: {};\n", self.dark.size_field));
        css.push_str(&format!("\t--dark-border: {};\n", self.dark.border));
        css.push_str(&format!("\t--dark-depth: {};\n", self.dark.depth));
        css.push_str(&format!("\t--dark-noise: {};\n", self.dark.noise));

        css.push_str("}\n");
        css
    }
}
