use improvie_logic::DynAppError;
use youtube::YtError;

#[derive(Debug, thiserror::Error)]
#[error("YouTube error: {0}")]
pub struct YtErrorWrapper(#[from] pub YtError);

impl DynAppError for YtErrorWrapper {
    fn error_kind(&self) -> &'static str {
        "YtError"
    }
}

improvie_logic::impl_serialize_for_dyn_app_error!(YtErrorWrapper,);
