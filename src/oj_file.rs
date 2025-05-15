use bon;

#[derive(Debug, Clone, bon::Builder)]
pub struct OjFile {
    pub path: String,
    pub path_slug: String,
    pub content: String,
}
