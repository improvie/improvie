#[derive(Debug, thiserror::Error)]
#[error("Database error: {0}")]
pub struct DbErr(pub sqlx::Error);

improvie_logic::impl_serialize_for_dyn_app_error!(DbErr);

impl improvie_logic::DynAppError for DbErr {
    fn error_kind(&self) -> &'static str {
        "DbErr"
    }
}
