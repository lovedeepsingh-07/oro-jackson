use crate::{context, error, oj_file};
use color_eyre::eyre;
use std::{fs, path};
use tracing;

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
        let parent_folder = path::Path::new(&curr_file.path_slug)
            .parent()
            .ok_or_else(|| {
                error::Error::NotFound(
                    "failed to get the parent folder for the given file".to_string(),
                )
            })?;
        let _ = fs::create_dir_all(parent_folder);
        fs::write(&curr_file.path_slug, &curr_file.content)?;
        tracing::info!("Successfully built {:#?}", curr_file.path_slug);
    }
    return Ok(());
}
