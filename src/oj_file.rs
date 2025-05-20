use bon;

#[derive(Debug, Clone, bon::Builder)]
pub struct OjFile {
    pub input_path: String,
    pub abs_input_path: String,
    pub output_path: String,
    pub content: String,
}

impl Default for OjFile {
    fn default() -> Self {
        return OjFile {
            input_path: String::new(),
            abs_input_path: String::new(),
            output_path: String::new(),
            content: String::new(),
        };
    }
}
