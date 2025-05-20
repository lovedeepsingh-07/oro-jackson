use crate::{context, error, oj_file};
use color_eyre::eyre;
use std::{fs, path};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StaticAssetsEmitterOptions {
    pub enable: bool,
}

#[derive(rust_embed::RustEmbed, Clone, Debug)]
#[folder = "_static/"]
pub struct StaticAssets;

pub fn get_embedded_file(filepath: String) -> eyre::Result<String, error::Error> {
    let file = StaticAssets::get(filepath.as_str()).ok_or_else(|| {
        error::Error::NotFound("no such embedded static file or directory".to_string())
    })?;
    let contents = String::from_utf8(file.data.to_vec())?;
    return Ok(contents);
}

pub fn static_assets_emitter(
    ctx: &context::Context,
    content_files: &Vec<oj_file::OjFile>,
) -> eyre::Result<(), error::Error> {
    let _ = content_files;

    // if this is a rebuild run, we don't need to write static assets again
    if ctx.is_rebuild == true {
        return Ok(());
    }

    let static_subdir_path = format!("{}/_static", ctx.build_args.output);

    for item in StaticAssets::iter() {
        let item_path = format!("{}/{}", static_subdir_path, item);

        let item_contents = get_embedded_file(item.to_string())?;

        let parent_folder = path::Path::new(&item_path).parent().ok_or_else(|| {
            error::Error::NotFound("failed to get the parent folder for the given file".to_string())
        })?;
        let _ = fs::create_dir_all(parent_folder);

        fs::write(&item_path, item_contents)?;
        if ctx.config.settings.logging == true {
            tracing::info!("Successfully built {:#?}", item_path);
        }
    }

    let theme_css_path = format!("{}/theme.css", static_subdir_path);

    let theme_css_contents: String = format!(
        r#":root {{
        --background-light: {};
        --foreground-light: {};
        --primary-light: {};
        --secondary-light: {};
        --accent-light: {};
        --neutral-light: {};
        --radius-light: {};
        --background-dark: {};
        --foreground-dark: {};
        --primary-dark: {};
        --secondary-dark: {};
        --accent-dark: {};
        --neutral-dark: {};
        --radius-dark: {};
    }}"#,
        ctx.config.theme.light.background,
        ctx.config.theme.light.foreground,
        ctx.config.theme.light.primary,
        ctx.config.theme.light.secondary,
        ctx.config.theme.light.accent,
        ctx.config.theme.light.neutral,
        ctx.config.theme.light.radius,
        ctx.config.theme.dark.background,
        ctx.config.theme.dark.foreground,
        ctx.config.theme.dark.primary,
        ctx.config.theme.dark.secondary,
        ctx.config.theme.dark.accent,
        ctx.config.theme.dark.neutral,
        ctx.config.theme.dark.radius,
    );

    fs::write(&theme_css_path, theme_css_contents)?;
    if ctx.config.settings.logging == true {
        tracing::info!("Successfully built {:#?}", theme_css_path);
    }

    return Ok(());
}
