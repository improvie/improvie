#[derive(Debug, thiserror::Error)]
pub enum YtError {
    #[error("failed getting video")]
    UrlMissing,
    #[error("failed to download video")]
    JoinError,
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("failed to create ffmpeg context: {0}")]
    Ffmpeg(Box<dyn std::error::Error + Send + Sync>),
}
