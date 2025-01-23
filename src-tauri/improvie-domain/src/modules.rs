use crate::repository::{health_check::HealthCheckRepository, items::ItemsRepository};

pub trait RepositoriesModule {
    type HealthCheckRepository: HealthCheckRepository;
    type ItemsRepository: ItemsRepository;

    fn health_check_repository(&self) -> &Self::HealthCheckRepository;
    fn items_repository(&self) -> &Self::ItemsRepository;
}
