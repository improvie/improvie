use std::{borrow::Borrow, fs::OpenOptions, path::PathBuf};

use sqlx::SqlitePool;

use crate::repository::MIGRATOR;

pub type DbTx = sqlx::Transaction<'static, sqlx::Sqlite>;

pub struct DbPool(SqlitePool);

impl Clone for DbPool {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[derive(Debug, thiserror::Error, more_convert::EnumName)]
pub enum InitDbError {
    #[error("create database error: {0}")]
    Db(#[from] sqlx::Error),
    #[error("create database error with io: {0}")]
    Io(#[from] std::io::Error),
}

impl DbPool {
    pub async fn new(data_dir: PathBuf) -> Result<Self, InitDbError> {
        std::fs::create_dir_all(&data_dir)?;
        let join = data_dir.join("data.sql");
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .open(&join)?;

        let connect = SqlitePool::connect(join.to_string_lossy().borrow()).await?;
        MIGRATOR
            .run(&connect)
            .await
            .map_err(|err| sqlx::Error::Migrate(Box::new(err)))?;
        Ok(Self(connect))
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
