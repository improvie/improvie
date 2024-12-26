use std::path::PathBuf;

use improvie_shared::AppResult;
use sqlx::SqlitePool;

pub struct DbPool(SqlitePool);

impl DbPool {
    pub async fn new(data_dir: PathBuf) -> AppResult<Self> {
        let path = data_dir.join("data.sql").to_str();
        let _ = path;
        todo!("handle error");
        // match path {
        //     Some(path) => Ok(Self(SqlitePool::connect(path).await)),
        //     None => Err(AppError("data_dir is not set".to_string())),
        // }
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
