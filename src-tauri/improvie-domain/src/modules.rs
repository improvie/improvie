use crate::repository::health_check::HealthCheckRepository;

pub trait RepositoriesModule {
    type HealthCheckRepository: HealthCheckRepository;

    fn health_check_repository(&self) -> &Self::HealthCheckRepository;
}
