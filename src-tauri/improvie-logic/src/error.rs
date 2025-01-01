pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("{1} already exist on {0}")]
    AlreadyExist(&'static str, String),
    #[error("{1} not found in {0}")]
    NotFound(&'static str, String),
    #[error("db error: {0}")]
    Db(#[from] sqlx::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
