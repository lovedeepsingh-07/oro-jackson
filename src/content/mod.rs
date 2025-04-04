// modules
#[cfg(test)]
mod tests;

use crate::{error, templates};
use ammonia;
use askama::Template;
use bon;
use std::{fs, path};
use walkdir;

#[bon::builder]
pub fn path_to_slug(input: String) -> String {
    return input
        .chars()
        .map(|c| if c.is_whitespace() { '-' } else { c })
        .collect();
}

#[bon::builder]
pub fn is_hidden_file(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

#[bon::builder]
pub fn is_markdown_file(file_path: &str) -> bool {
    return file_path.ends_with(".md");
}

#[bon::builder]
pub fn generate_html(markdown_content: &str) -> Result<String, String> {
    let mut output_html = String::new();

    let mut options = pulldown_cmark::Options::empty();
    options.insert(pulldown_cmark::Options::ENABLE_MATH);
    options.insert(pulldown_cmark::Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
    options.insert(pulldown_cmark::Options::ENABLE_GFM);
    options.insert(pulldown_cmark::Options::ENABLE_WIKILINKS);
    let parser = pulldown_cmark::Parser::new_ext(markdown_content, options);
    pulldown_cmark::html::push_html(&mut output_html, parser);

    match (templates::PageTemplate {
        content: ammonia::clean(&output_html),
    }
    .render())
    {
        Ok(safe_html) => return Ok(safe_html),
        Err(e) => return Err(e.to_string()),
    };
}

#[bon::builder]
pub fn build_content(
    content_folder_path: &str,
    output_folder_path: &str,
    input_path_string: &str,
) -> Result<(), error::ContentError> {
    let input_path = path::Path::new(input_path_string);

    if input_path.is_file() {
        let file_path = input_path_string;
        let html_file = path_to_slug()
            .input(
                file_path
                    .replace(
                        fs::canonicalize(path::Path::new(content_folder_path))
                            .unwrap()
                            .to_string_lossy()
                            .to_string()
                            .as_str(),
                        fs::canonicalize(path::Path::new(output_folder_path))
                            .unwrap()
                            .to_string_lossy()
                            .to_string()
                            .as_str(),
                    )
                    .replace(".md", ".html"),
            )
            .call();

        let folder = match path::Path::new(&html_file).parent() {
            Some(safe_folder) => safe_folder,
            None => return Err(error::ContentError::ParentFolderCreateError),
        };
        let _ = fs::create_dir_all(folder);

        let markdown_content = match fs::read_to_string(file_path) {
            Ok(safe_md_content) => safe_md_content,
            Err(e) => return Err(error::ContentError::FileContentReadError(e.to_string())),
        };
        let html = match generate_html().markdown_content(&markdown_content).call() {
            Ok(safe_html) => safe_html,
            Err(e) => return Err(error::ContentError::HTMLRenderError(e.to_string())),
        };

        match fs::write(&html_file, html) {
            Ok(_) => {}
            Err(e) => return Err(error::ContentError::FileWriteError(e.to_string())),
        };

        println!("Successfully built {:#?}", html_file);

        return Ok(());
    } else if input_path.is_dir() {
        let mut output_files: Vec<String> = Vec::new();
        for entry in walkdir::WalkDir::new(input_path)
            .into_iter()
            .filter_entry(|e| !is_hidden_file().entry(e).call())
        {
            match entry {
                Ok(safe_entry) => {
                    let entry_path = safe_entry.path().display().to_string();

                    if is_markdown_file().file_path(&entry_path).call() {
                        output_files.push(safe_entry.path().display().to_string())
                    }
                }
                Err(e) => {
                    println!(
                        "WARNING: Unable to access file from content folder, Error: {:#?}",
                        e.to_string()
                    )
                }
            }
        }

        let mut html_files: Vec<String> = Vec::with_capacity(output_files.len());

        for md_file in output_files {
            let html_file = path_to_slug()
                .input(
                    md_file
                        .replace(content_folder_path, output_folder_path)
                        .replace(".md", ".html"),
                )
                .call();

            let folder = match path::Path::new(&html_file).parent() {
                Some(safe_folder) => safe_folder,
                None => return Err(error::ContentError::ParentFolderCreateError),
            };
            let _ = fs::create_dir_all(folder);

            let markdown_content = match fs::read_to_string(md_file) {
                Ok(safe_md_content) => safe_md_content,
                Err(e) => return Err(error::ContentError::FileContentReadError(e.to_string())),
            };
            let html = match generate_html().markdown_content(&markdown_content).call() {
                Ok(safe_html) => safe_html,
                Err(e) => return Err(error::ContentError::HTMLRenderError(e.to_string())),
            };

            match fs::write(&html_file, html) {
                Ok(_) => {}
                Err(e) => return Err(error::ContentError::FileWriteError(e.to_string())),
            };
            println!("Successfully built {:#?}", html_file);

            html_files.push(html_file.to_string());
        }

        return Ok(());
    } else {
        return Err(error::ContentError::InvalidInputPath(format!(
            "the provided folder, {:#?} is not a file or a directory directory",
            input_path_string
        )));
    }
}
