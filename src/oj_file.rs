use crate::frontmatter;
use bon;

#[derive(Debug, Clone, bon::Builder)]
pub struct OjFile {
    pub frontmatter: frontmatter::Frontmatter,
    pub input_path: vfs::VfsPath,
    pub output_path: vfs::VfsPath,
    pub content: String,
}
