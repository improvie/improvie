pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("{1} already exist on {0}")]
    AlreadyExist(&'static str, String),
}
