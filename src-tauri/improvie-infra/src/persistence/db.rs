use improvie_logic::{DynAppError, DynAppResult};
use sea_orm::{ConnectionTrait, DbConn, TransactionTrait};
use std::{fs::OpenOptions, path::PathBuf};

#[derive(Clone, Copy)]
pub enum DbConnection<'a> {
    Pool(&'a DbPool),
    Tx(&'a DbTx),
}

macro_rules! internal {
    ($(@$await:ident)?$self:ident, $fn:ident$(($($expr:expr)+))?) => {
        match $self {
            Self::Pool(pool) => {
                pool.0.$fn($( $($expr)+ )?)$(.$await)?
            }
            Self::Tx(tx) => {
                tx.0.$fn($( $($expr)+ )?)$(.$await)?
            }
        }
    };
}

#[async_trait::async_trait]
impl sea_orm::ConnectionTrait for DbConnection<'_> {
    fn get_database_backend(&self) -> sea_orm::DbBackend {
        internal!(self, get_database_backend)
    }

    async fn execute(
        &self,
        stmt: sea_orm::Statement,
    ) -> Result<sea_orm::ExecResult, sea_orm::DbErr> {
        internal!(@await self, execute(stmt))
    }

    async fn execute_unprepared(&self, sql: &str) -> Result<sea_orm::ExecResult, sea_orm::DbErr> {
        internal!(@await self, execute_unprepared(sql))
    }

    async fn query_one(
        &self,
        stmt: sea_orm::Statement,
    ) -> Result<Option<sea_orm::QueryResult>, sea_orm::DbErr> {
        internal!(@await self, query_one(stmt))
    }

    async fn query_all(
        &self,
        stmt: sea_orm::Statement,
    ) -> Result<Vec<sea_orm::QueryResult>, sea_orm::DbErr> {
        internal!(@await self, query_all(stmt))
    }

    fn support_returning(&self) -> bool {
        internal!(self, support_returning)
    }

    fn is_mock_connection(&self) -> bool {
        internal!(self, is_mock_connection)
    }
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
pub struct DbPool(DbConn);

impl DbPool {
    pub fn pool(&self) -> &DbConn {
        &self.0
    }

    pub fn backend(&self) -> sea_orm::DbBackend {
        self.0.get_database_backend()
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

pub struct DbTx(sea_orm::DatabaseTransaction);

impl DbTx {
    fn new(tx: sea_orm::DatabaseTransaction) -> Self {
        Self(tx)
    }

    pub fn tx(&self) -> &sea_orm::DatabaseTransaction {
        &self.0
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
    Db(#[from] sea_orm::error::DbErr),
    #[error("create database error with io: {0}")]
    Io(#[from] std::io::Error),
}

impl DynAppError for InitDbError {
    fn error_kind(&self) -> &'static str {
        "InitDbError"
    }
}

pub(crate) static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../migrations");

impl DbPool {
    pub async fn new(data_dir: PathBuf) -> Result<Self, InitDbError> {
        std::fs::create_dir_all(&data_dir)?;
        let join = data_dir.join("data.sql");
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .open(&join)?;

        let mut option = sea_orm::ConnectOptions::new(format!("sqlite:{}", join.display()));
        #[cfg(debug_assertions)]
        if option_env!("ENABLE_SQLX_LOG").is_some_and(|v| v.parse::<bool>().is_ok_and(|b| b)) {
            log::debug!("enable sqlx logging");
        } else {
            log::debug!(
                "disable sqlx logging for readability. Set `ENABLE_SQLX_LOG=true` to enable it."
            );
            option.sqlx_logging(false);
        };

        Self::new_internal(option).await
    }

    async fn new_internal(option: sea_orm::ConnectOptions) -> Result<Self, InitDbError> {
        let connect = sea_orm::SqlxSqliteConnector::connect(option)
            .await
            .map_err(InitDbError::Db)?;

        MIGRATOR
            .run(connect.get_sqlite_connection_pool())
            .await
            .map_err(|err| InitDbError::Db(sea_orm::error::DbErr::Migration(err.to_string())))?;
        Ok(Self(connect))
    }
}

impl DbPool {
    #[cfg(feature = "test")]
    pub async fn new_test() -> Self {
        const DB_URL: &str = "sqlite::memory:";
        let option = sea_orm::ConnectOptions::new(DB_URL);

        #[allow(clippy::unwrap_used)]
        Self::new_internal(option).await.unwrap()
    }
}
