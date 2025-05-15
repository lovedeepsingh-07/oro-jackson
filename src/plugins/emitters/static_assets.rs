use crate::{context, error, oj_file};
use color_eyre::eyre;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StaticAssetsEmitterOptions {
    pub enable: bool,
}

pub fn static_assets_emitter(
    ctx: &context::Context,
    content_files: &Vec<oj_file::OjFile>,
) -> eyre::Result<(), error::Error> {
    let _ = content_files;
    let _ = ctx;
    return Ok(());
}
