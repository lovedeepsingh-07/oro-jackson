use crate::{context, error, frontmatter, oj_file};
use color_eyre::eyre;
use regex::Regex;

#[cfg(test)]
pub mod tests;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FrontmatterTransformerOptions {
    pub enable: bool,
}

pub fn extract_frontmatter(
    input: &str,
) -> eyre::Result<Option<(String, String, String)>, error::Error> {
    let default_exp =
        Regex::new(r"^[[:space:]]*\-\-\-\r?\n((?s).*?(?-s))\-\-\-\r?\n((?s).*(?-s))$")?;
    let toml_exp = Regex::new(r"^[[:space:]]*\+\+\+\r?\n((?s).*?(?-s))\+\+\+\r?\n((?s).*(?-s))$")?;
    let mut captures: Option<regex::Captures> = None;
    let mut expr_type = String::from("yaml");

    if default_exp.is_match(input) {
        captures = default_exp.captures(input);
    }

    if captures.is_none() && toml_exp.is_match(input) {
        expr_type = String::from("toml");
        captures = toml_exp.captures(input);
    }

    if let Some(cap) = captures {
        let res = (cap[1].trim().to_string(), cap[2].trim().to_string());
        return Ok(Some((expr_type, res.0, res.1)));
    }

    Ok(None)
}

pub fn frontmatter_transformer<'a>(
    ctx: &'a context::Context,
    content_files: &'a mut Vec<oj_file::OjFile>,
) -> eyre::Result<&'a mut Vec<oj_file::OjFile>, error::Error> {
    let _ = ctx;
    for curr_file in content_files.iter_mut() {
        let mut markdown_content: String = String::new();
        match extract_frontmatter(&curr_file.content)? {
            Some((expr_type, frontmatter_content, markd_content)) => {
                markdown_content = markd_content;
                match expr_type.as_str() {
                    "yaml" => {
                        curr_file.frontmatter.source =
                            frontmatter::FrontmatterSource::Yaml(serde_yaml::from_str::<
                                serde_yaml::Value,
                            >(
                                &frontmatter_content
                            )?)
                    }
                    "toml" => {
                        curr_file.frontmatter.source = frontmatter::FrontmatterSource::Toml(
                            toml::from_str::<toml::Value>(&frontmatter_content)?,
                        )
                    }
                    _ => {
                        return Err(eyre::eyre!(error::Error::FrontmatterError(String::from(
                            "invalid frontmatter expression type"
                        ))))?;
                    }
                }
            }
            None => {}
        }
        curr_file.content = markdown_content;
    }

    return Ok(content_files);
}
