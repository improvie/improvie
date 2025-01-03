use std::sync::Arc;

use improvie_domain::repository::health_check::HealthCheckRepository;

use crate::persistence::db::DbPool;

pub struct HealthCheckRepositoryImpl {
    db: Arc<DbPool>,
}

impl HealthCheckRepositoryImpl {
    pub fn new(db: Arc<DbPool>) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl HealthCheckRepository for HealthCheckRepositoryImpl {
    async fn health_check(&self) -> improvie_logic::AppResult<()> {
        let _ = self.db;
        Ok(())
    }
}
