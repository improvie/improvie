use sqlx::SqlitePool;

pub struct DbPool(SqlitePool);

impl DbPool {
    pub fn new() -> std::io::Result<Self> {
        todo!()
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
