use improvie_logic::{BoxDynAppError, DynAppError, impl_serialize_for_dyn_app_error};
use more_convert::VariantName;

#[derive(Debug, thiserror::Error, more_convert::VariantName)]
#[variant_name(prefix = "Yt")]
pub enum YtError {
    #[error("failed to download video")]
    JoinError,
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("http error: {0}")]
    Http(#[from] rusty_ytdl::VideoError),
    #[error("failed to create ffmpeg context: {0}")]
    Ffmpeg(Box<dyn std::error::Error + Send + Sync>),
}

impl DynAppError for YtError {
    fn error_kind(&self) -> &'static str {
        self.variant_name()
    }
}

impl_serialize_for_dyn_app_error!(YtError);
