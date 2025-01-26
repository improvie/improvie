use crate::repository::{
    health_check::HealthCheckRepository, items::ItemsRepository, playlists::PlaylistsRepository,
};

pub trait RepositoriesModule {
    type HealthCheckRepository: HealthCheckRepository;
    type ItemsRepository: ItemsRepository;
    type PlaylistsRepository: PlaylistsRepository;

    fn health_check_repository(&self) -> &Self::HealthCheckRepository;
    fn items_repository(&self) -> &Self::ItemsRepository;
    fn playlists_repository(&self) -> &Self::PlaylistsRepository;
}
