use std::path::PathBuf;

use improvie_shared::{AppError, AppResult};
use sqlx::SqlitePool;

pub struct DbPool(SqlitePool);

impl DbPool {
    pub async fn new(data_dir: PathBuf) -> AppResult<Self> {
        std::fs::create_dir_all(&data_dir)?;
        let join = data_dir.join("data.sql");
        std::fs::File::create(&join)?;
        match join.to_str() {
            Some(path) => Ok(Self(SqlitePool::connect(path).await?)),
            None => Err(AppError::NotFound("path", String::from("data_dir"))),
        }
    }

    pub fn pool(&self) -> SqlitePool {
        self.0.clone()
    }

    pub async fn tx(&self) -> sqlx::Result<sqlx::Transaction<'static, sqlx::Sqlite>> {
        self.0.begin().await
    }
}

#[cfg(test)]
impl DbPool {
    pub fn with_pool(pool: SqlitePool) -> std::sync::Arc<Self> {
        std::sync::Arc::new(Self(pool))
    }
}
