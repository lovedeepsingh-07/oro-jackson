use bon;

#[derive(Debug, Clone, bon::Builder)]
pub struct OjFile {
    pub input_path: String,
    pub abs_input_path: String,
    pub output_path: String,
    pub content: String,
}
