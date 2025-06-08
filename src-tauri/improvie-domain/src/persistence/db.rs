use improvie_logic::DynAppResult;

pub trait DbConnection<'a>: Send + Sync {
    type DbPool: DbPool<DbTx = Self::DbTx>;
    type DbTx: DbTx;

    fn new_pool(pool: &'a Self::DbPool) -> Self;
    fn new_tx(tx: &'a Self::DbTx) -> Self;
}

pub trait DbPool: Send + Sync + 'static {
    type DbConnection<'a>: DbConnection<'a, DbPool = Self, DbTx = Self::DbTx>;
    type DbTx: DbTx;

    fn connection(&self) -> Self::DbConnection<'_> {
        Self::DbConnection::new_pool(self)
    }

    /// Begins a new database transaction.
    fn begin(&self) -> impl Future<Output = DynAppResult<Self::DbTx>>;
}

pub trait DbTx: Send + Sync + 'static {
    type DbConnection<'a>: DbConnection<'a, DbPool = Self::DbPool, DbTx = Self>;
    type DbPool: DbPool<DbTx = Self>;

    fn connection(&self) -> Self::DbConnection<'_> {
        Self::DbConnection::new_tx(self)
    }

    /// Commits the transaction.
    fn commit(self) -> impl Future<Output = DynAppResult<()>>;

    /// Rolls back the transaction.
    fn rollback(self) -> impl Future<Output = DynAppResult<()>>;
}
