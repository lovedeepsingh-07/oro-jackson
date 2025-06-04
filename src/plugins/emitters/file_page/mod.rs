use crate::{context, error, oj_file, web};
use color_eyre::eyre;
use leptos::prelude::RenderHtml;
use tracing;

#[cfg(test)]
pub mod tests;

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
        if curr_file.input_path.filename() == "index.md" {
            continue;
        };

        let file_page_frontmatter = web::pages::PageFrontmatter::new(ctx, &curr_file);
        let file_page_html =
            web::pages::file_page::FilePage(web::pages::file_page::FilePageProps {
                content: curr_file.content.clone(),
                frontmatter: file_page_frontmatter,
            })
            .to_html();

        curr_file.output_path.parent().create_dir_all()?;
        let mut f = curr_file.output_path.create_file()?;
        f.write_all(file_page_html.as_bytes())?;

        if ctx.config.settings.logging == true {
            tracing::info!("Successfully built {:#?}", curr_file.output_path.as_str());
        }
    }
    return Ok(());
}
