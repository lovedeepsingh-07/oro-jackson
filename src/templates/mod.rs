use crate::content;
use askama::Template;
use bon;

#[derive(Template, bon::Builder)]
#[template(path = "file.html")]
pub struct FileTemplate {
    pub content: String,
}

#[derive(Template, bon::Builder)]
#[template(path = "folder.html")]
pub struct FolderTemplate {
    pub subfiles: Vec<content::FolderTemplateChildLink>,
}
