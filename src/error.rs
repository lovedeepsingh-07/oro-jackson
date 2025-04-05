use thiserror::Error;

#[derive(Debug, Error)]
pub enum WebStateError {}

#[derive(Debug, Error)]
pub enum ContentError {
    #[error("provided input path is not a valid file or a directory, Error: {0}")]
    InvalidInputPath(String),

    #[error("failed to read file contents, Error: {0}")]
    FileContentReadError(String),

    #[error("failed to render HTML template, Error: {0}")]
    HTMLRenderError(String),

    #[error("failed to create parent folder")]
    ParentFolderCreateError,

    #[error("failed to write content to file, Error: {0}")]
    FileWriteError(String),
}
