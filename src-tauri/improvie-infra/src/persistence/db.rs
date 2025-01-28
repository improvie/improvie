use std::path::PathBuf;

use improvie_logic::{AppError, AppResult};
use sqlx::SqlitePool;

use crate::repository::MIGRATOR;

pub struct DbPool(SqlitePool);

impl Clone for DbPool {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl DbPool {
    pub async fn new(data_dir: PathBuf) -> AppResult<Self> {
        std::fs::create_dir_all(&data_dir)?;
        let join = data_dir.join("data.sql");
        std::fs::File::create(&join)?;
        match join.to_str() {
            Some(path) => {
                let connect = SqlitePool::connect(path).await?;
                MIGRATOR
                    .run(&connect)
                    .await
                    .map_err(|err| sqlx::Error::Migrate(Box::new(err)))?;
                Ok(Self(connect))
            }
            None => Err(AppError::NotFound("path", String::from("data_dir"))),
        }
    }

    pub fn pool(&self) -> SqlitePool {
        self.0.clone()
    }

    pub async fn begin(&self) -> sqlx::Result<sqlx::Transaction<'static, sqlx::Sqlite>> {
        self.0.begin().await
    }
}

#[cfg(test)]
impl DbPool {
    pub fn with_pool(pool: SqlitePool) -> Self {
        Self(pool)
    }
}
