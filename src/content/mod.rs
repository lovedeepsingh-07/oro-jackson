#[cfg(test)]
mod tests;

use crate::{error, templates};
use ammonia;
use askama::Template;
use bon;
use rust_embed;
use std::{fs, path};
use walkdir;

#[derive(Debug, bon::Builder)]
pub struct FolderTemplateChildLink {
    pub name: String,
    pub href: String,
}

#[derive(rust_embed::RustEmbed, Clone, Debug)]
#[folder = "_static/"]
pub struct StaticAssets;

pub fn get_embedded_file(filepath: String) -> Option<Result<String, String>> {
    match StaticAssets::get(filepath.as_str()) {
        Some(file_content) => {
            return Some(match String::from_utf8(file_content.data.to_vec()) {
                Ok(safe_value) => Ok(safe_value),
                Err(e) => Err(e.to_string()),
            });
        }
        None => {
            return None;
        }
    }
}

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
pub fn generate_file_html(markdown_content: &str) -> Result<String, String> {
    let mut output_html = String::new();

    let mut options = pulldown_cmark::Options::empty();
    options.insert(pulldown_cmark::Options::ENABLE_MATH);
    options.insert(pulldown_cmark::Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
    options.insert(pulldown_cmark::Options::ENABLE_GFM);
    options.insert(pulldown_cmark::Options::ENABLE_WIKILINKS);
    let parser = pulldown_cmark::Parser::new_ext(markdown_content, options);
    pulldown_cmark::html::push_html(&mut output_html, parser);

    match templates::FileTemplate::builder()
        .content(ammonia::clean(&output_html))
        .build()
        .render()
    {
        Ok(safe_html) => return Ok(safe_html),
        Err(e) => return Err(e.to_string()),
    };
}

#[bon::builder]
pub fn generate_folder_html(subfiles: Vec<FolderTemplateChildLink>) -> Result<String, String> {
    match templates::FolderTemplate::builder()
        .subfiles(subfiles)
        .build()
        .render()
    {
        Ok(safe_html) => return Ok(safe_html),
        Err(e) => return Err(e.to_string()),
    };
}

#[bon::builder]
pub fn build_static_assets(output_folder_path: String) -> Result<(), error::ContentError> {
    let static_subdir_path = format!("{}/_static", output_folder_path);
    for item in StaticAssets::iter() {
        let item_path = format!("{}/{}", static_subdir_path, item);

        let item_contents = match get_embedded_file(item.to_string()) {
            Some(some_file_contents) => match some_file_contents {
                Ok(safe_file_contents) => safe_file_contents,
                Err(e) => {
                    return Err(error::ContentError::FileContentToStringConvertError(
                        e.to_string(),
                    ));
                }
            },
            None => {
                return Err(error::ContentError::StaticFileNotFoundError(
                    item.to_string(),
                ))
            }
        };

        let folder = match path::Path::new(&item_path).parent() {
            Some(safe_folder) => safe_folder,
            None => return Err(error::ContentError::ParentFolderCreateError),
        };
        let _ = fs::create_dir_all(folder);

        match fs::write(&item_path, item_contents) {
            Ok(_) => {}
            Err(e) => return Err(error::ContentError::FileWriteError(e.to_string())),
        };
    }

    return Ok(());
}

#[bon::builder]
pub fn build_curr_folder_index_file(
    output_folder_string: String,
    input_folder_string: String,
) -> Result<(), error::ContentError> {
    if output_folder_string != input_folder_string {
        let input_folder_path = path::Path::new(&input_folder_string);
        let ignores = Vec::from([".git", ".obsidian", "index"]);

        let mut curr_folder_subfiles: Vec<FolderTemplateChildLink> = Vec::new();
        if input_folder_path.is_dir() {
            match fs::read_dir(input_folder_path) {
                Ok(folder_entries) => {
                    for folder_entry in folder_entries {
                        match folder_entry {
                            Ok(folder_entry) => {
                                let entry_path = folder_entry.path();
                                let entry_name =
                                    folder_entry.file_name().to_string_lossy().to_string();
                                if !ignores.contains(&entry_name.as_str()) {
                                    if entry_path.is_dir() {
                                        curr_folder_subfiles.push(
                                            FolderTemplateChildLink::builder()
                                                .name(entry_name.replace(".html", ""))
                                                .href(
                                                    entry_path
                                                        .to_string_lossy()
                                                        .to_string()
                                                        .replace(&output_folder_string, "")
                                                        .replace(".html", ""),
                                                )
                                                .build(),
                                        );
                                    } else if entry_path.is_file() {
                                        curr_folder_subfiles.push(
                                            FolderTemplateChildLink::builder()
                                                .name(entry_name.replace(".html", ""))
                                                .href(
                                                    entry_path
                                                        .to_string_lossy()
                                                        .to_string()
                                                        .replace(&output_folder_string, "")
                                                        .replace(".html", ""),
                                                )
                                                .build(),
                                        );
                                    } else {
                                        return Err(error::ContentError::InvalidInputPath(format!("the provided folder, {:#?} is not a file or a directory directory",entry_path.to_string_lossy().to_string(),)));
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!(
                                "WARNING: Unable to access file from content folder, Error: {:#?}",
                                e.to_string()
                            );
                            }
                        }
                    }
                }
                Err(e) => {
                    return Err(error::ContentError::ReadDirectoryEntryError(e.to_string()));
                }
            }
        }

        let index_html = match templates::FolderTemplate::builder()
            .subfiles(curr_folder_subfiles)
            .build()
            .render()
        {
            Ok(safe_html) => safe_html,
            Err(e) => return Err(error::ContentError::HTMLRenderError(e.to_string())),
        };

        let index_file_location = format!(
            "{}/index.html",
            input_folder_path.to_string_lossy().to_string()
        );
        match fs::write(index_file_location, index_html) {
            Ok(_) => {}
            Err(e) => return Err(error::ContentError::FileWriteError(e.to_string())),
        };
    }

    return Ok(());
}

#[bon::builder]
pub fn build_index_files(output_folder_path: String) -> Result<(), error::ContentError> {
    for entry in walkdir::WalkDir::new(&output_folder_path)
        .into_iter()
        .filter_entry(|e| !is_hidden_file().entry(e).call())
    {
        match entry {
            Ok(safe_entry) => {
                let entry_path = safe_entry.path().display().to_string();
                if safe_entry.path().is_dir() {
                    match build_curr_folder_index_file()
                        .output_folder_string(output_folder_path.to_string())
                        .input_folder_string(entry_path.clone())
                        .call()
                    {
                        Ok(_) => {}
                        Err(e) => {
                            return Err(error::ContentError::IndexFilesBuildError(e.to_string()))
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "WARNING: Unable to access file from content folder, Error: {:#?}",
                    e.to_string()
                )
            }
        }
    }

    return Ok(());
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
                        match fs::canonicalize(path::Path::new(content_folder_path)) {
                            Ok(safe_stuff) => safe_stuff,
                            Err(e) => {
                                return Err(error::ContentError::FilePathCanonicalizeError(
                                    e.to_string(),
                                ))
                            }
                        }
                        .to_string_lossy()
                        .to_string()
                        .as_str(),
                        match fs::canonicalize(path::Path::new(output_folder_path)) {
                            Ok(safe_stuff) => safe_stuff,
                            Err(e) => {
                                return Err(error::ContentError::FilePathCanonicalizeError(
                                    e.to_string(),
                                ))
                            }
                        }
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
        let html = match generate_file_html()
            .markdown_content(&markdown_content)
            .call()
        {
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
                    eprintln!(
                        "WARNING: Unable to access file from content folder, Error: {:#?}",
                        e.to_string()
                    )
                }
            }
        }

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
            let html = match generate_file_html()
                .markdown_content(&markdown_content)
                .call()
            {
                Ok(safe_html) => safe_html,
                Err(e) => return Err(error::ContentError::HTMLRenderError(e.to_string())),
            };

            match fs::write(&html_file, html) {
                Ok(_) => {}
                Err(e) => return Err(error::ContentError::FileWriteError(e.to_string())),
            };
            println!("Successfully built {:#?}", html_file);
        }

        return Ok(());
    } else {
        return Err(error::ContentError::InvalidInputPath(format!(
            "the provided folder, {:#?} is not a file or a directory directory",
            input_path_string
        )));
    }
}
