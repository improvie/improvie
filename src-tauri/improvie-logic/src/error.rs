use serde::{Serialize, Serializer};

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Debug, thiserror::Error, Serialize)]
#[serde(tag = "kind", content = "message")]
#[serde(rename_all = "camelCase")]
pub enum AppError {
    #[error("{1} already exist on {0}")]
    AlreadyExist(&'static str, String),

    #[error("{1} not found in {0}")]
    NotFound(&'static str, String),

    #[error("invalid {1} in {0}")]
    Invalid(&'static str, String),

    #[error("db error: {0}")]
    #[serde(serialize_with = "to_string")]
    Db(#[from] sqlx::Error),

    #[error("io error: {0}")]
    #[serde(serialize_with = "to_string")]
    Io(#[from] std::io::Error),
}

#[inline]
fn to_string<S, T>(t: &T, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: ToString,
{
    s.serialize_str(t.to_string().as_ref())
}
