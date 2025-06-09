use crate::{context, error, oj_file};
use color_eyre::eyre;

#[cfg(test)]
pub mod tests;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StaticAssetsEmitterOptions {
    pub enable: bool,
}

#[derive(rust_embed::RustEmbed, Clone, Debug)]
#[folder = "_static/"]
pub struct StaticAssets;

pub fn get_embedded_static_file(filepath: String) -> eyre::Result<String, error::Error> {
    let file = StaticAssets::get(filepath.as_str()).ok_or_else(|| {
        error::Error::NotFound("no such embedded static file or directory".to_string())
    })?;
    let content = String::from_utf8(file.data.to_vec())?;
    return Ok(content);
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

    let static_subdir_path = ctx.build_args.output.join("_static")?;
    static_subdir_path.create_dir()?;

    for item in StaticAssets::iter() {
        let item_path = static_subdir_path.join(item.to_string())?;

        let item_content = get_embedded_static_file(item.to_string())?;

        let parent_folder = item_path.parent();
        parent_folder.create_dir_all()?;

        let mut f = item_path.create_file()?;
        f.write_all(item_content.as_bytes())?;

        if ctx.config.logging == true {
            tracing::info!("Successfully built {:#?}", item_path.as_str());
        }
    }

    let theme_css_path = static_subdir_path.join("theme.css")?;
    let mut f = theme_css_path.create_file()?;
    f.write_all(ctx.theme.as_bytes())?;

    if ctx.config.logging == true {
        tracing::info!("Successfully built {:#?}", theme_css_path.as_str());
    }

    return Ok(());
}
