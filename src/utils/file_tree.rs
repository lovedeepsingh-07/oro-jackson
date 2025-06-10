use crate::{error, utils};
use bon;
use color_eyre::eyre;
use serde;
use vfs;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TreeNode {
    File(TreeNodeFile),
    Folder(TreeNodeFolder),
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TreeNodeFile {
    pub name: String,
    pub href: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TreeNodeFolder {
    pub name: String,
    pub href: String,
    pub children: Vec<TreeNode>,
}

// here two separate vectors created to store files and folders separately but then later combined
// in order to ensure that folders appear before the files in the ouput vector, the only thing it
// does is that it ensure that folders are on top of the files when the file tree is rendered
#[bon::builder]
pub fn map_folder(input_path: vfs::VfsPath) -> eyre::Result<Vec<TreeNode>, error::Error> {
    let mut output_folders: Vec<TreeNode> = Vec::new();
    let mut output_files: Vec<TreeNode> = Vec::new();

    for entry in input_path.read_dir()? {
        if entry.is_file()?
            && !utils::is_hidden_file().file_path(&entry).call()
            && utils::is_markdown_file().file_path(&entry).call()
            && entry.filename() != "index.md"
        {
            output_files.push(TreeNode::File(TreeNodeFile {
                name: entry.filename().replace(".md", ""),
                href: entry.as_str().to_string().replace(".md", ""),
            }))
        }
        if entry.is_dir()?
            && !utils::is_hidden_file().file_path(&entry).call()
            && !utils::is_empty_dir().dir_path(&entry).call()
        {
            output_folders.push(TreeNode::Folder(TreeNodeFolder {
                name: entry.filename(),
                href: entry.as_str().to_string(),
                children: map_folder().input_path(entry).call()?,
            }))
        }
    }

    output_folders.extend(output_files);
    return Ok(output_folders);
}
