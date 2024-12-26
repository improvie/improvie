use std::sync::Arc;

use improvie_domain::{
    modules::RepositoriesModule, repository::health_check::HealthCheckRepository,
};
use improvie_shared::AppResult;

pub struct HealthCheckUseCase<R: RepositoriesModule> {
    repository: Arc<R>,
}

impl<R: RepositoriesModule> HealthCheckUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    pub async fn health_check(&self) -> AppResult<()> {
        self.repository
            .health_check_repository()
            .health_check()
            .await
    }
}
