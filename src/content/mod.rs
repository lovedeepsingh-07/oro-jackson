use crate::{error, templates};
use askama::Template;
use bon;
use color_eyre::eyre::{self, WrapErr};
use rust_embed;
use std::{fs, path};
use tracing;
use walkdir;

#[cfg(test)]
mod tests;

#[derive(Debug, bon::Builder)]
pub struct FolderTemplateChildLink {
    pub name: String,
    pub href: String,
}

#[derive(rust_embed::RustEmbed, Clone, Debug)]
#[folder = "_static/"]
pub struct StaticAssets;

pub fn get_embedded_file(filepath: String) -> eyre::Result<String, error::Error> {
    let file = StaticAssets::get(filepath.as_str()).ok_or_else(|| {
        error::Error::NotFound("no such embedded static file or directory".to_string())
    })?;
    let contents = String::from_utf8(file.data.to_vec())?;
    return Ok(contents);
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
pub fn generate_html_for_file_page(markdown_content: &str) -> eyre::Result<String, error::Error> {
    let mut output_html = String::new();

    let mut options = pulldown_cmark::Options::empty();
    options.insert(pulldown_cmark::Options::ENABLE_MATH);
    options.insert(pulldown_cmark::Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
    options.insert(pulldown_cmark::Options::ENABLE_GFM);
    options.insert(pulldown_cmark::Options::ENABLE_WIKILINKS);
    let parser = pulldown_cmark::Parser::new_ext(markdown_content, options);
    pulldown_cmark::html::push_html(&mut output_html, parser);

    let html = templates::FileTemplate::builder()
        .content(output_html)
        .build()
        .render()
        .wrap_err("failed to render file page HTML template")?;

    return Ok(html);
}

#[bon::builder]
pub fn generate_html_for_folder_page(
    subfiles: Vec<FolderTemplateChildLink>,
) -> eyre::Result<String, error::Error> {
    let html = templates::FolderTemplate::builder()
        .subfiles(subfiles)
        .build()
        .render()
        .wrap_err("failed to render folder page HTML template")?;
    return Ok(html);
}

#[bon::builder]
pub fn build_static_assets(output_folder_path: String) -> eyre::Result<(), error::Error> {
    let static_subdir_path = format!("{}/_static", output_folder_path);
    for item in StaticAssets::iter() {
        let item_path = format!("{}/{}", static_subdir_path, item);

        let item_contents = get_embedded_file(item.to_string())?;

        let parent_folder = path::Path::new(&item_path).parent().ok_or_else(|| {
            error::Error::NotFound("failed to get the parent folder for the given file".to_string())
        })?;
        let _ = fs::create_dir_all(parent_folder);

        fs::write(&item_path, item_contents)?;
    }

    return Ok(());
}

#[bon::builder]
pub fn build_curr_folder_index_file(
    output_folder_string: String,
    input_folder_string: String,
) -> eyre::Result<(), error::Error> {
    if input_folder_string == output_folder_string {
        return Ok(());
    }

    let input_folder_path = path::Path::new(&input_folder_string);

    if !input_folder_path.is_dir() {
        return Err(error::Error::InvalidInput(format!(
            "provided input path is not a valid file or a directory, input: {}",
            input_folder_string
        )))?;
    }
    let ignores = Vec::from([".git", ".obsidian", "index"]);
    let mut curr_folder_subfiles: Vec<FolderTemplateChildLink> = Vec::new();

    let folder_entries = fs::read_dir(input_folder_path)?;

    for folder_entry in folder_entries {
        let folder_entry = match folder_entry {
            Ok(folder_entry) => folder_entry,
            Err(e) => {
                tracing::warn!(
                    "unable to access file from content folder, error: {:#?}",
                    e.to_string()
                );
                continue;
            }
        };
        let entry_path = folder_entry.path();
        let entry_name = folder_entry.file_name().to_string_lossy().to_string();

        if ignores.contains(&entry_name.as_str()) {
            continue;
        }

        let href = entry_path
            .to_string_lossy()
            .to_string()
            .replace(&output_folder_string, "")
            .replace(".html", "");

        if entry_path.is_dir() || entry_path.is_file() {
            curr_folder_subfiles.push(
                FolderTemplateChildLink::builder()
                    .name(entry_name.replace(".html", ""))
                    .href(href)
                    .build(),
            );
        } else {
            return Err(error::Error::InvalidInput(format!(
                "provided folder entry is neither a file nor a directory, input: {}",
                input_folder_string
            )))?;
        }
    }

    let index_html = generate_html_for_folder_page()
        .subfiles(curr_folder_subfiles)
        .call()?;

    let index_file_location = format!(
        "{}/index.html",
        input_folder_path.to_string_lossy().to_string()
    );

    fs::write(index_file_location, index_html)?;

    return Ok(());
}

#[bon::builder]
pub fn build_index_files(output_folder_path: String) -> eyre::Result<(), error::Error> {
    let entries = walkdir::WalkDir::new(&output_folder_path)
        .into_iter()
        .filter_entry(|e| !is_hidden_file().entry(e).call());
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                tracing::warn!(
                    "unable to access file from content folder, error: {:#?}",
                    e.to_string()
                );
                continue;
            }
        };
        if !entry.path().is_dir() {
            continue;
        }
        let entry_path_string = entry.path().display().to_string();
        build_curr_folder_index_file()
            .output_folder_string(output_folder_path.to_string())
            .input_folder_string(entry_path_string.clone())
            .call()?;
    }

    return Ok(());
}

#[bon::builder]
pub fn build_content(
    content_folder_path: &str,
    output_folder_path: &str,
    input_path_string: &str,
) -> eyre::Result<(), error::Error> {
    let input_path = path::Path::new(input_path_string);

    let content_canon = fs::canonicalize(content_folder_path)?
        .to_string_lossy()
        .to_string();

    // create all the parents of the `output_build_path` recursively because if the output folder
    // does not exist, the following `fs::canonicalize()` will throw an error
    let _ = fs::create_dir_all(path::Path::new(output_folder_path));
    let output_canon = fs::canonicalize(output_folder_path)?
        .to_string_lossy()
        .to_string();

    if input_path.is_file() {
        // build only the current file
        process_single_file()
            .input_path_string(input_path_string)
            .content_folder_path(&content_canon)
            .output_folder_path(&output_canon)
            .call()?;
        return Ok(());
    } else if input_path.is_dir() {
        // build all the subfiles
        let mut output_files: Vec<String> = Vec::new();

        let folder_entries = walkdir::WalkDir::new(input_path)
            .into_iter()
            .filter_entry(|e| !is_hidden_file().entry(e).call());

        for entry in folder_entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    tracing::warn!(
                        "unable to access file from content folder, error: {:#?}",
                        e.to_string()
                    );
                    continue;
                }
            };
            let entry_path = entry.path().display().to_string();

            if is_markdown_file().file_path(&entry_path).call() {
                output_files.push(entry_path)
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

            let parent_folder = path::Path::new(&html_file).parent().ok_or_else(|| {
                error::Error::NotFound(
                    "failed to get the parent folder for the given file".to_string(),
                )
            })?;
            let _ = fs::create_dir_all(parent_folder);

            let markdown_content = fs::read_to_string(md_file)?;
            let html = generate_html_for_file_page()
                .markdown_content(&markdown_content)
                .call()?;

            fs::write(&html_file, html)?;
            tracing::info!("Successfully built {:#?}", html_file);
        }

        return Ok(());
    } else {
        return Err(error::Error::InvalidInput(format!(
            "provided folder entry is neither a file nor a directory, input: {}",
            input_path_string
        )))?;
    }
}

#[bon::builder]
pub fn process_single_file(
    content_folder_path: &str,
    output_folder_path: &str,
    input_path_string: &str,
) -> eyre::Result<(), error::Error> {
    let file_path = input_path_string;
    let html_file = path_to_slug()
        .input(
            file_path
                .replace(content_folder_path, output_folder_path)
                .replace(".md", ".html"),
        )
        .call();

    let parent_folder = path::Path::new(&html_file).parent().ok_or_else(|| {
        error::Error::NotFound("failed to get the parent folder for the given file".to_string())
    })?;

    let _ = fs::create_dir_all(parent_folder);

    let markdown_content = fs::read_to_string(file_path)?;

    let html = generate_html_for_file_page()
        .markdown_content(&markdown_content)
        .call()?;

    fs::write(&html_file, html)?;

    tracing::info!("Successfully built {:#?}", html_file);
    return Ok(());
}
