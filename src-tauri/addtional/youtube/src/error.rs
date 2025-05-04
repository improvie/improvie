use improvie_logic::impl_serialize_for_dyn_app_error;

#[derive(Debug, thiserror::Error, more_convert::VariantName)]
#[variant_name(prefix = "Yt")]
pub enum YtError {
    #[error("not found document dir")]
    NotFoundDocumentDir,
    #[error("invalid url")]
    InvalidUrl,
    #[error("failed to download video")]
    JoinError,
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("http error: {0}")]
    Http(#[from] rusty_ytdl::VideoError),
    #[error("failed to create content: {0}")]
    SaveError(#[from] improvie_logic::AppError),
    #[error("failed to create ffmpeg context: {0}")]
    Ffmpeg(#[from] ez_ffmpeg::error::Error),
}

impl_serialize_for_dyn_app_error!(YtError);
