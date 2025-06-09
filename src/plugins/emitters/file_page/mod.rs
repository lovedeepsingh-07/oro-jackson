use crate::{context, error, frontmatter, oj_file, pages};
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
        if curr_file.input_path.filename() == "index.md" && !curr_file.input_path.parent().is_root()
        {
            continue;
        };

        let file_page_frontmatter = frontmatter::Frontmatter::new(ctx, &curr_file);
        let file_page_html = pages::file_page::FilePage(pages::file_page::FilePageProps {
            content: curr_file.content.clone(),
            show_file_explorer: ctx.config.file_explorer,
            frontmatter: file_page_frontmatter,
            project_title: ctx.config.title.clone(),
            file_tree: ctx.file_tree.clone(),
        })
        .to_html();

        curr_file.output_path.parent().create_dir_all()?;
        let mut f = curr_file.output_path.create_file()?;
        f.write_all(file_page_html.as_bytes())?;

        if ctx.config.logging == true {
            tracing::info!("Successfully built {:#?}", curr_file.output_path.as_str());
        }
    }
    return Ok(());
}
