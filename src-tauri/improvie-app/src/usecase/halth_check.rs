use improvie_domain::{
    modules::RepositoriesModule, repository::health_check::HealthCheckRepository,
};
use improvie_logic::AppResult;

super::def_use_case!(HealthCheckUseCase);

impl<R: RepositoriesModule> HealthCheckUseCase<R> {
    pub async fn health_check(&self) -> AppResult<bool> {
        self.repository
            .health_check_repository()
            .health_check()
            .await
            .map(|v| v.is_some())
    }
}
