use improvie_shared::AppResult;

#[async_trait::async_trait]
pub trait HealthCheckRepository {
    async fn health_check(&self) -> AppResult<()>;
}
