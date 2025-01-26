use improvie_domain::{modules::RepositoriesModule, repository::playlists::PlaylistsRepository};
use improvie_logic::{
    model::playlist::{Play, Playlist},
    AppResult, Uuid,
};

pub struct PlaylistsUseCase<R: RepositoriesModule> {
    repository: R,
}

impl<R: RepositoriesModule> PlaylistsUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn get_playlists(&self) -> AppResult<Vec<Playlist>> {
        self.repository.playlists_repository().get_playlists().await
    }

    pub async fn get_plays(&self) -> AppResult<Vec<Play>> {
        self.repository.playlists_repository().get_plays().await
    }

    pub async fn get_favorite_playlists(&self) -> AppResult<Vec<Uuid>> {
        self.repository
            .playlists_repository()
            .get_favorite_playlists()
            .await
    }
}
