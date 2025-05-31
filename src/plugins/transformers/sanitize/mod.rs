use crate::{context, error, oj_file};
use ammonia;
use color_eyre::eyre;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SanitizeTransformerOptions {
    pub enable: bool,
}

pub fn sanitize_transformer<'a>(
    ctx: &'a context::Context,
    content_files: &'a mut Vec<oj_file::OjFile>,
) -> eyre::Result<&'a mut Vec<oj_file::OjFile>, error::Error> {
    let _ = ctx;
    let mut default_ammonia_builder = ammonia::Builder::default();
    let html_sanitizer = default_ammonia_builder
        .add_tags(maplit::hashset!["code", "span"])
        .add_tag_attributes("code", maplit::hashset!["class"])
        .add_tag_attributes("span", maplit::hashset!["class"]);
    for curr_file in content_files.iter_mut() {
        curr_file.content = html_sanitizer.clean(&curr_file.content).to_string();
    }

    return Ok(content_files);
}
