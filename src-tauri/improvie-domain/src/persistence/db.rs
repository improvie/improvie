use improvie_logic::DynAppResult;

pub trait DbConnection<'a>: Send + Sync {
    type DbPool: DbPool<DbTx = Self::DbTx>;
    type DbTx: DbTx;

    fn new_pool(pool: &'a Self::DbPool) -> Self;
    fn new_tx(tx: &'a Self::DbTx) -> Self;
}

#[async_trait::async_trait]
pub trait DbPool: Send + Sync + 'static {
    type DbConnection<'a>: DbConnection<'a, DbPool = Self, DbTx = Self::DbTx>;
    type DbTx: DbTx;

    fn connection(&self) -> Self::DbConnection<'_> {
        Self::DbConnection::new_pool(self)
    }

    /// Begins a new database transaction.
    async fn begin(&self) -> DynAppResult<Self::DbTx>;
}

#[async_trait::async_trait]
pub trait DbTx: Send + Sync + 'static {
    type DbConnection<'a>: DbConnection<'a, DbPool = Self::DbPool, DbTx = Self>;
    type DbPool: DbPool<DbTx = Self>;

    fn connection(&self) -> Self::DbConnection<'_> {
        Self::DbConnection::new_tx(self)
    }

    /// Commits the transaction.
    async fn commit(self) -> DynAppResult<()>;

    /// Rolls back the transaction.
    async fn rollback(self) -> DynAppResult<()>;
}
