use improvie_domain::{
    model::health_check::SettingsModel, repository::health_check::HealthCheckRepository,
};

use crate::{model::health_check::SettingsRow, persistence::db::DbPool};

pub struct HealthCheckRepositoryImpl {
    db: DbPool,
}

impl HealthCheckRepositoryImpl {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl HealthCheckRepository for HealthCheckRepositoryImpl {
    async fn health_check(&self) -> improvie_logic::AppResult<Option<SettingsModel>> {
        let settings: Option<SettingsRow> =
            sqlx::query_as::<_, SettingsRow>("SELECT id, created_at FROM settings")
                .fetch_optional(&self.db.pool())
                .await?;
        Ok(settings.map(Into::into))
    }
}
