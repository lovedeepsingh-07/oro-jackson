use crate::{context, error, oj_file};
use color_eyre::eyre;

#[cfg(test)]
pub mod tests;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MarkdownTransformerOptions {
    pub enable: bool,
}

pub fn markdown_transformer<'a>(
    ctx: &'a context::Context,
    content_files: &'a mut Vec<oj_file::OjFile>,
) -> eyre::Result<&'a mut Vec<oj_file::OjFile>, error::Error> {
    let _ = ctx;

    let mut options = pulldown_cmark::Options::empty();
    options.insert(pulldown_cmark::Options::ENABLE_MATH);
    options.insert(pulldown_cmark::Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
    options.insert(pulldown_cmark::Options::ENABLE_GFM);
    options.insert(pulldown_cmark::Options::ENABLE_TABLES);
    options.insert(pulldown_cmark::Options::ENABLE_FOOTNOTES);
    options.insert(pulldown_cmark::Options::ENABLE_SUPERSCRIPT);
    options.insert(pulldown_cmark::Options::ENABLE_SUBSCRIPT);
    options.insert(pulldown_cmark::Options::ENABLE_DEFINITION_LIST);
    options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
    options.insert(pulldown_cmark::Options::ENABLE_TASKLISTS);

    // TODO: the internal wikilinks don't really work, I want two custom types of WIKILINKS
    // - absolute: these types would follow links from the base of the content folder
    // - relative; these types would follow links relative to the file in which they appear
    // also the wikilink to a folder takes you to folder/index route, when it should just take you
    // to the folder/ route, but the index is added for some reason infront of the route, it is not
    // intended behaviour
    options.insert(pulldown_cmark::Options::ENABLE_WIKILINKS);

    {
        for curr_file in content_files.iter_mut() {
            let mut output_html = String::new();
            // this parser thing seems to be an iteratable representation of the Markdown AST
            let parser = pulldown_cmark::Parser::new_ext(&curr_file.content, options);
            pulldown_cmark::html::push_html(&mut output_html, parser);
            curr_file.content = output_html.clone();
        }
    }

    return Ok(content_files);
}
