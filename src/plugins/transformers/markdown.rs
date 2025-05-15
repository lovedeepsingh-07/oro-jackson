use crate::{error, oj_file};
use color_eyre::eyre;
use markdown;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MarkdownTransformerOptions {
    pub enable: bool,
}

pub fn markdown_transformer(
    content_files: &mut Vec<oj_file::OjFile>,
) -> eyre::Result<&mut Vec<oj_file::OjFile>, error::Error> {
    let markdown_options = markdown::Options {
        parse: markdown::ParseOptions {
            constructs: markdown::Constructs {
                math_flow: true,
                math_text: true,
                frontmatter: true,
                ..markdown::Constructs::gfm()
            },
            math_text_single_dollar: true,
            ..markdown::ParseOptions::gfm()
        },
        ..markdown::Options::gfm()
    };
    {
        for curr_file in content_files.iter_mut() {
            curr_file.content =
                markdown::to_html_with_options(&curr_file.content, &markdown_options)
                    .map_err(|e| error::Error::MarkdownError(e.to_string()))?;
        }
    }
    return Ok(content_files);
}
