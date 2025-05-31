use crate::{context, error, oj_file, utils, web};
use color_eyre::eyre;
use leptos::prelude::RenderHtml;
use std::{fs, path};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FolderPageEmitterOptions {
    pub enable: bool,
    pub show_folder_page_children: bool,
}

#[derive(Debug, Clone, bon::Builder)]
pub struct FolderPageChildLink {
    pub name: String,
    pub href: String,
}

#[bon::builder]
pub fn prepare_folder_files(
    ctx: &context::Context,
    content_files: &Vec<oj_file::OjFile>,
) -> eyre::Result<Vec<oj_file::OjFile>> {
    let mut folder_files: Vec<oj_file::OjFile> = Vec::new();

    for curr_file in content_files {
        let curr_file_parents = get_parent_folders()
            .curr_file_input_path(&curr_file.input_path)
            .content_base_path(&ctx.build_args.content)
            .call()?;
        for curr_parent_path in curr_file_parents {
            if folder_files.iter().any(|f| {
                f.input_path
                    == path::Path::new(&curr_parent_path)
                        .join("index.md")
                        .to_string_lossy()
                        .to_string()
            }) {
                continue;
            }
            let mut curr_index_file: oj_file::OjFile = oj_file::OjFile::default();
            if let Some(folder_file) = content_files.iter().find(|cf| {
                cf.input_path
                    == get_folder_index_file_path()
                        .folder_path(&curr_parent_path)
                        .call()
            }) {
                curr_index_file = folder_file.clone();
            } else {
                // the frontmatter stays a null value
                curr_index_file.input_path = path::Path::new(&curr_parent_path)
                    .join("index.md")
                    .to_string_lossy()
                    .to_string();
                curr_index_file.abs_input_path = fs::canonicalize(&curr_parent_path)?
                    .join("index.md")
                    .to_string_lossy()
                    .to_string();
                curr_index_file.output_path = path::Path::new(
                    &curr_parent_path.replace(&ctx.build_args.content, &ctx.build_args.output),
                )
                .join("index.html")
                .to_string_lossy()
                .to_string();
                // the content stays an empty string
            }
            folder_files.push(curr_index_file);
        }
    }
    return Ok(folder_files);
}

// TODO: this feels incredibly inefficient
pub fn folder_page_emitter(
    ctx: &context::Context,
    content_files: &Vec<oj_file::OjFile>,
) -> eyre::Result<(), error::Error> {
    let folder_files = prepare_folder_files()
        .ctx(ctx)
        .content_files(content_files)
        .call()?;

    for index_file in folder_files {
        let parent_folder = path::Path::new(&index_file.input_path)
            .parent()
            .ok_or_else(|| {
                error::Error::NotFound(
                    "failed to get the parent folder for the given file".to_string(),
                )
            })?;

        let folder_children = get_children()
            .index_file_parent_folder(&parent_folder.to_string_lossy().to_string())
            .output_folder_string(&ctx.build_args.content)
            .call()?;

        let folder_page_frontmatter = web::pages::PageFrontmatter::new(ctx, &index_file);
        let folder_page_html =
            web::pages::folder_page::FolderPage(web::pages::folder_page::FolderPageProps {
                frontmatter: folder_page_frontmatter,
                content: index_file.content.clone(),
                subfiles: folder_children,
                show_folder_page_children: ctx
                    .config
                    .plugins
                    .emitters
                    .folder_page
                    .show_folder_page_children,
            })
            .to_html();

        let _ = fs::create_dir_all(
            path::Path::new(&index_file.output_path)
                .parent()
                .ok_or_else(|| {
                    error::Error::NotFound(
                        "failed to get the parent folder for the given file".to_string(),
                    )
                })?,
        );
        fs::write(&index_file.output_path, &folder_page_html)?;

        if ctx.config.settings.logging == true {
            tracing::info!("Successfully built {:#?}", index_file.output_path);
        }
    }
    return Ok(());
}

#[bon::builder]
pub fn get_parent_folders(
    curr_file_input_path: &str,
    content_base_path: &str,
) -> eyre::Result<Vec<String>> {
    let mut folder_name = path::Path::new(curr_file_input_path)
        .parent()
        .ok_or_else(|| {
            error::Error::NotFound("failed to get the parent folder for the given file".to_string())
        })?;

    let mut parent_folders: Vec<String> = vec![folder_name.to_string_lossy().to_string()];

    while folder_name != path::Path::new(content_base_path) {
        folder_name = folder_name.parent().ok_or_else(|| {
            error::Error::NotFound("failed to get the parent folder for the given file".to_string())
        })?;
        parent_folders.push(folder_name.to_string_lossy().to_string());
    }
    return Ok(parent_folders);
}

#[bon::builder]
pub fn get_children(
    index_file_parent_folder: &str,
    output_folder_string: &str,
) -> eyre::Result<Vec<FolderPageChildLink>> {
    let mut children: Vec<FolderPageChildLink> = Vec::new();

    if !path::Path::new(&index_file_parent_folder).exists() {
        return Ok(children);
    }

    for child_entry in fs::read_dir(index_file_parent_folder)? {
        let child_entry = match child_entry {
            Ok(folder_entry) => folder_entry,
            Err(e) => {
                tracing::warn!(
                    "unable to access file from content folder, error: {:#?}",
                    e.to_string()
                );
                continue;
            }
        };

        let child_entry_file_name = child_entry.file_name();
        let child_entry_path = child_entry.path();

        if child_entry_file_name == "_static"
            || (child_entry_path.is_file()
                && (child_entry_file_name == "index.md"
                    || !utils::is_markdown_file()
                        .file_path(&child_entry_file_name.to_string_lossy().to_string())
                        .call()))
            || utils::is_hidden_file()
                .entry_file_name(child_entry_file_name.to_string_lossy().to_string())
                .call()
        {
            continue;
        }

        if child_entry_path.is_dir() {
            match fs::read_dir(&child_entry_path) {
                Ok(dir_iter) => {
                    // Collect all entries, filtering out errors
                    let entries: Vec<_> = dir_iter.filter_map(Result::ok).collect();

                    // Skip if the directory is empty or only contains a `.keep` file
                    if entries.is_empty()
                        || (entries.len() == 1 && entries[0].file_name() == ".keep")
                    {
                        continue;
                    }
                }
                Err(e) => {
                    tracing::warn!(
                        "unable to read directory {:?}, error: {:#?}",
                        child_entry_path,
                        e.to_string()
                    );
                    continue;
                }
            }
        };

        let child_entry_path_string = child_entry_path
            .to_string_lossy()
            .to_string()
            .replace(".md", "");

        children.push(
            FolderPageChildLink::builder()
                .name(
                    child_entry_path_string
                        .replace(index_file_parent_folder, "")
                        .replace("/", ""),
                )
                .href(child_entry_path_string.replace(output_folder_string, ""))
                .build(),
        );
    }

    return Ok(children);
}

#[bon::builder]
pub fn get_folder_index_file_path(folder_path: &str) -> String {
    return path::Path::new(&folder_path)
        .join("index.md")
        .to_string_lossy()
        .to_string();
}
