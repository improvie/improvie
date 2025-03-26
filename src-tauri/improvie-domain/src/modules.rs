use crate::repository::{
    health_check::HealthCheckRepository, items::ItemsRepository, playlists::PlaystsRepository,
};

pub trait RepositoriesModule: Clone + Send + Sync + Sized + 'static {
    type HealthCheckRepository: HealthCheckRepository;
    type ItemsRepository: ItemsRepository;
    type PlaystsRepository: PlaystsRepository;

    fn health_check_repository(&self) -> &Self::HealthCheckRepository;
    fn items_repository(&self) -> &Self::ItemsRepository;
    fn playsts_repository(&self) -> &Self::PlaystsRepository;
}
