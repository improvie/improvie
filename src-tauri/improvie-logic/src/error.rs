use more_convert::EnumName;
use serde::{ser::SerializeStruct, Serializer};

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Debug, thiserror::Error, more_convert::EnumName)]
pub enum AppError {
    #[error("{1} already exist on {0}")]
    AlreadyExist(&'static str, String),

    #[error("{1} not found in {0}")]
    NotFound(&'static str, String),

    #[error("invalid {1} in {0}")]
    Invalid(&'static str, String),

    #[error("db error: {0}")]
    Db(#[from] sqlx::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut serde_state = serializer.serialize_struct("AppError", 2)?;
        serde_state.serialize_field("kind", self.enum_name())?;
        serde_state.serialize_field("message", &self.to_string())?;
        serde_state.end()
    }
}
