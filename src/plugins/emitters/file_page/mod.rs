use crate::{context, error, oj_file, web};
use color_eyre::eyre;
use leptos::prelude::RenderHtml;
use std::{fs, path};
use tracing;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FilePageEmitterOptions {
    pub enable: bool,
}

pub fn file_page_emitter(
    ctx: &context::Context,
    content_files: &Vec<oj_file::OjFile>,
) -> eyre::Result<(), error::Error> {
    let _ = ctx;
    for curr_file in content_files {
        if path::Path::new(&curr_file.input_path)
            .file_name()
            .ok_or_else(|| {
                error::Error::NotFound(
                    "failed to get the parent folder for the given file".to_string(),
                )
            })?
            .to_string_lossy()
            .to_string()
            == String::from("index.md")
        {
            continue;
        };
        let parent_folder = path::Path::new(&curr_file.output_path)
            .parent()
            .ok_or_else(|| {
                error::Error::NotFound(
                    "failed to get the parent folder for the given file".to_string(),
                )
            })?;

        let file_page_frontmatter = web::pages::PageFrontmatter::new(ctx, &curr_file);
        let file_page_html =
            web::pages::file_page::FilePage(web::pages::file_page::FilePageProps {
                content: curr_file.content.clone(),
                frontmatter: file_page_frontmatter,
            })
            .to_html();

        let _ = fs::create_dir_all(parent_folder);
        fs::write(&curr_file.output_path, &file_page_html)?;

        if ctx.config.settings.logging == true {
            tracing::info!("Successfully built {:#?}", curr_file.output_path);
        }
    }
    return Ok(());
}
