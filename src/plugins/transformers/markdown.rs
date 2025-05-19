use crate::{error, oj_file};
use color_eyre::eyre;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MarkdownTransformerOptions {
    pub enable: bool,
}

pub fn markdown_transformer(
    content_files: &mut Vec<oj_file::OjFile>,
) -> eyre::Result<&mut Vec<oj_file::OjFile>, error::Error> {
    let mut options = pulldown_cmark::Options::empty();
    options.insert(pulldown_cmark::Options::ENABLE_MATH);
    options.insert(pulldown_cmark::Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
    options.insert(pulldown_cmark::Options::ENABLE_GFM);
    // the internal wikilinks don't really work, I want two custom types of WIKILINKS
    // - absolute: these types would follow links from the base of the content folder
    // - relative; these types would follow links relative to the file in which they appear
    // options.insert(pulldown_cmark::Options::ENABLE_WIKILINKS);

    {
        for curr_file in content_files.iter_mut() {
            let mut output_html = String::new();
            // this parser thing seems to be an iteratable representation of the Markdown AST
            let parser = pulldown_cmark::Parser::new_ext(&curr_file.content, options);
            pulldown_cmark::html::push_html(&mut output_html, parser);
            curr_file.content = output_html;
        }
    }

    return Ok(content_files);
}
