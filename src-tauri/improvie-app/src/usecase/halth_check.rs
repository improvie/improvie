use improvie_domain::{
    modules::RepositoriesModule, repository::health_check::HealthCheckRepository,
};
use improvie_logic::AppResult;

pub struct HealthCheckUseCase<R: RepositoriesModule> {
    repository: R,
}

impl<R: RepositoriesModule> HealthCheckUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn health_check(&self) -> AppResult<bool> {
        self.repository
            .health_check_repository()
            .health_check()
            .await
            .map(|v| v.is_some())
    }
}
