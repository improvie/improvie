use improvie_logic::AppResult;

use crate::model::health_check::SettingsModel;

#[async_trait::async_trait]
pub trait HealthCheckRepository {
    async fn health_check(&self) -> AppResult<Option<SettingsModel>>;
}
