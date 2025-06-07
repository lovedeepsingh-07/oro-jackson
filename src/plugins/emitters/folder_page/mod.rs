use crate::{context, error, frontmatter, oj_file, pages, utils};
use color_eyre::eyre;
use leptos::prelude::RenderHtml;

#[cfg(test)]
pub mod tests;

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
        let parent_folder = index_file.input_path.parent();

        let folder_children = get_children()
            .index_file_parent_folder(parent_folder)
            .call()?;

        let folder_page_frontmatter = frontmatter::Frontmatter::new(ctx, &index_file);
        let folder_page_html =
            pages::folder_page::FolderPage(pages::folder_page::FolderPageProps {
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

        index_file.output_path.parent().create_dir_all()?;
        let mut f = index_file.output_path.create_file()?;
        f.write_all(folder_page_html.as_bytes())?;

        if ctx.config.settings.logging == true {
            tracing::info!("Successfully built {:#?}", index_file.output_path.as_str());
        }
    }
    return Ok(());
}

#[bon::builder]
pub fn prepare_folder_files(
    ctx: &context::Context,
    content_files: &Vec<oj_file::OjFile>,
) -> eyre::Result<Vec<oj_file::OjFile>> {
    let mut folder_files: Vec<oj_file::OjFile> = Vec::new();

    for curr_file in content_files {
        let curr_file_parents = get_parent_folders()
            .curr_file_input_path(curr_file.input_path.clone())
            .content_base_path(ctx.build_args.content.clone())
            .call()?;
        for curr_parent_path in curr_file_parents {
            let curr_index_path = curr_parent_path.join("index.md")?;

            if folder_files.iter().any(|f| f.input_path == curr_index_path) {
                continue;
            }
            if let Some(folder_file) = content_files
                .iter()
                .find(|cf| cf.input_path == curr_index_path)
            {
                folder_files.push(folder_file.clone());
            } else {
                let index_file = oj_file::OjFile {
                    frontmatter: frontmatter::Frontmatter::default(),
                    input_path: curr_index_path,
                    output_path: ctx
                        .build_args
                        .output
                        .join(curr_parent_path.as_str())?
                        .join("index.html")?,
                    content: String::new(),
                };
                folder_files.push(index_file);
            }
        }
    }
    return Ok(folder_files);
}

#[bon::builder]
pub fn get_parent_folders(
    curr_file_input_path: vfs::VfsPath,
    content_base_path: vfs::VfsPath,
) -> eyre::Result<Vec<vfs::VfsPath>> {
    let mut folder = curr_file_input_path.parent();

    let mut parent_folders: Vec<vfs::VfsPath> = vec![folder.clone()];

    while folder != content_base_path {
        folder = folder.parent();
        parent_folders.push(folder.clone());
    }
    return Ok(parent_folders);
}

#[bon::builder]
pub fn get_children(
    index_file_parent_folder: vfs::VfsPath,
) -> eyre::Result<Vec<FolderPageChildLink>> {
    let mut children: Vec<FolderPageChildLink> = Vec::new();

    if !index_file_parent_folder.exists()? {
        return Ok(children);
    }

    for child_entry in index_file_parent_folder.read_dir()? {
        let child_entry_file_name = child_entry.filename();

        if child_entry_file_name == "_static"
            || (child_entry.is_file()?
                && (child_entry_file_name == "index.md"
                    || !utils::is_markdown_file().file_path(&child_entry).call()))
            || utils::is_hidden_file().file_path(&child_entry).call()
        {
            continue;
        }

        if child_entry.is_dir()? {
            match child_entry.read_dir() {
                Ok(dir_iter) => {
                    // Collect all entries, filtering out errors
                    let entries: Vec<_> = dir_iter.collect();

                    // Skip if the directory is empty or only contains a `.keep` file
                    if entries.is_empty()
                        || (entries.len() == 1 && entries[0].filename() == ".keep")
                    {
                        continue;
                    }
                }
                Err(e) => {
                    tracing::warn!(
                        "unable to read directory {:?}, error: {:#?}",
                        child_entry,
                        e.to_string()
                    );
                    continue;
                }
            }
        };

        let child_entry_path_string = child_entry.as_str().replace(".md", "");

        children.push(
            FolderPageChildLink::builder()
                .name(
                    child_entry_path_string
                        .replace(index_file_parent_folder.as_str(), "")
                        .replace("/", ""),
                )
                .href(child_entry_path_string)
                .build(),
        );
    }

    return Ok(children);
}
