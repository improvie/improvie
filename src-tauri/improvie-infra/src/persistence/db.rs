use improvie_logic::{DynAppError, DynAppResult};
use sqlx::ConnectOptions;
use std::{fs::OpenOptions, path::PathBuf};

use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};

use crate::repository::MIGRATOR;

pub enum DbConnection<'a> {
    Pool(&'a DbPool),
    Tx(&'a DbTx),
}

impl<'a> improvie_domain::persistence::db::DbConnection<'a> for DbConnection<'a> {
    type DbPool = DbPool;
    type DbTx = DbTx;

    fn new_pool(pool: &'a Self::DbPool) -> Self {
        Self::Pool(pool)
    }

    fn new_tx(tx: &'a Self::DbTx) -> Self {
        Self::Tx(tx)
    }
}

#[derive(Clone)]
pub struct DbPool(SqlitePool);

impl DbPool {
    pub fn pool(&self) -> SqlitePool {
        self.0.clone()
    }

    pub async fn begin(&self) -> DynAppResult<DbTx> {
        let result = self.0.begin().await;
        match result {
            Ok(tx) => Ok(DbTx::new(tx)),
            Err(e) => Err(improvie_logic::DbErr(e).into()),
        }
    }
}

#[async_trait::async_trait]
impl improvie_domain::persistence::db::DbPool for DbPool {
    type DbConnection<'a> = DbConnection<'a>;
    type DbTx = DbTx;

    async fn begin(&self) -> DynAppResult<Self::DbTx> {
        self.begin().await
    }
}

pub struct DbTx(sqlx::Transaction<'static, sqlx::Sqlite>);

impl DbTx {
    fn new(tx: sqlx::Transaction<'static, sqlx::Sqlite>) -> Self {
        Self(tx)
    }

    pub fn tx(&mut self) -> &mut sqlx::Transaction<'static, sqlx::Sqlite> {
        &mut self.0
    }

    pub async fn commit(self) -> DynAppResult<()> {
        self.0
            .commit()
            .await
            .map_err(|e| improvie_logic::DbErr(e).into())
    }

    pub async fn rollback(self) -> DynAppResult<()> {
        self.0
            .rollback()
            .await
            .map_err(|e| improvie_logic::DbErr(e).into())
    }
}

impl AsMut<sqlx::SqliteConnection> for DbTx {
    fn as_mut(&mut self) -> &mut sqlx::SqliteConnection {
        self.0.as_mut()
    }
}

#[async_trait::async_trait]
impl improvie_domain::persistence::db::DbTx for DbTx {
    type DbConnection<'a> = DbConnection<'a>;
    type DbPool = DbPool;

    async fn commit(self) -> improvie_logic::DynAppResult<()> {
        self.commit().await
    }

    async fn rollback(self) -> improvie_logic::DynAppResult<()> {
        self.rollback().await
    }
}

#[derive(Debug, thiserror::Error)]
pub enum InitDbError {
    #[error("create database error: {0}")]
    Db(#[from] sqlx::Error),
    #[error("create database error with io: {0}")]
    Io(#[from] std::io::Error),
}

impl DynAppError for InitDbError {
    fn error_kind(&self) -> &'static str {
        "InitDbError"
    }
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

        let option = SqliteConnectOptions::new().filename(&join);
        #[cfg(debug_assertions)]
        let option =
            if option_env!("ENABLE_SQLX_LOG").is_some_and(|v| v.parse::<bool>().is_ok_and(|b| b)) {
                log::debug!("enable sqlx logging");
                option
            } else {
                log::debug!(
                    "disable sqlx logging for readability. Set `ENABLE_SQLX_LOG=true` to enable it."
                );
                option.disable_statement_logging()
            };
        let connect = SqlitePool::connect_with(option).await?;
        MIGRATOR
            .run(&connect)
            .await
            .map_err(|err| sqlx::Error::Migrate(Box::new(err)))?;
        Ok(Self(connect))
    }
}

#[cfg(test)]
impl DbPool {
    pub fn with_pool(pool: SqlitePool) -> Self {
        Self(pool)
    }
}
