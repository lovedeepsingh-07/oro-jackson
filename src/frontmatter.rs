use crate::oj_file;
use serde_yaml;
use std::path;
use toml;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Frontmatter {
    pub title: String,
    #[serde(skip)]
    pub source: FrontmatterSource,
}

impl Default for Frontmatter {
    fn default() -> Self {
        Self {
            title: String::new(),
            source: FrontmatterSource::default(),
        }
    }
}

impl Frontmatter {
    pub fn new(
        content_path: vfs::VfsPath,
        project_title: String,
        curr_file: &oj_file::OjFile,
    ) -> Self {
        let mut curr_frontmatter: Frontmatter = Frontmatter::default();
        curr_frontmatter.title = curr_file
            .frontmatter
            .source
            .get_title()
            .content_path(content_path)
            .project_title(project_title)
            .curr_file(curr_file)
            .call();
        return curr_frontmatter;
    }
}

#[derive(Debug, Clone)]
pub enum FrontmatterSource {
    Toml(toml::Value),
    Yaml(serde_yaml::Value),
}

impl Default for FrontmatterSource {
    fn default() -> Self {
        return FrontmatterSource::Yaml(serde_yaml::Value::Null);
    }
}

#[bon::bon]
impl FrontmatterSource {
    #[builder]
    pub fn get_title(
        &self,
        content_path: vfs::VfsPath,
        project_title: String,
        curr_file: &oj_file::OjFile,
    ) -> String {
        match self {
            FrontmatterSource::Yaml(frontmatter_source) => {
                if frontmatter_source.is_null() {
                    return get_title_from_file(
                        content_path,
                        project_title,
                        curr_file.input_path.clone(),
                    );
                } else if let Some(title) = frontmatter_source.get("title") {
                    if let Ok(ok_title) = serde_yaml::from_value::<String>(title.clone()) {
                        return ok_title;
                    }
                }
            }
            FrontmatterSource::Toml(frontmatter_source) => {
                if let Some(title) = frontmatter_source.get("title") {
                    return title.to_string();
                }
            }
        }
        return get_title_from_file(content_path, project_title, curr_file.input_path.clone());
    }
}

pub fn get_title_from_file(
    content_path: vfs::VfsPath,
    project_title: String,
    input_path: vfs::VfsPath,
) -> String {
    if let Some(curr_file_name) = path::PathBuf::from(input_path.filename()).file_stem() {
        if curr_file_name == "index" {
            let parent_path = input_path.parent();
            if parent_path == content_path {
                return project_title.clone();
            } else {
                let parent_name = parent_path.filename();
                return format!("Folder: {}", parent_name);
            }
        }
        return curr_file_name
            .to_str()
            .unwrap_or_else(|| {
                tracing::warn!("failed to compute the page title from file name");
                "null"
            })
            .to_string();
    }
    return "null".to_string();
}
