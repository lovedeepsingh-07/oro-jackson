use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("failed to create web state, Error: {0}")]
    WebStateError(String),

    #[error("failed to build content, Error: {0}")]
    ContentBuildError(String),

    #[error("failed to build static assets, Error: {0}")]
    StaticAssetsBuildError(String),

    #[error("Failed to bind TCP Listener to address, Error: {0}")]
    TCPListenerBindError(String),

    #[error("Failed to start the serrver listener, Error: {0}")]
    ServerListenerStartError(String),
}

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

    #[error("failed to build static assets, Error: {0}")]
    StaticAssetsBuildError(String),

    #[error("failed to convert file contents into readable string format, Error: {0}")]
    FileContentToStringConvertError(String),

    #[error("no such file in the embedded static assets, File: {0}")]
    StaticFileNotFoundError(String),
}
