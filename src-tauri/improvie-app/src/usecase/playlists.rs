use std::collections::HashMap;

use improvie_domain::{modules::RepositoriesModule, repository::playlists::PlaylistsRepository};
use improvie_logic::{
    AppResult,
    model::playlist::{Playlist, PlaylistFolder},
};
use uuid::Uuid;

pub struct PlaylistsUseCase<R: RepositoriesModule> {
    repository: R,
}

impl<R: RepositoriesModule> PlaylistsUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn get_playlist_folders(&self) -> AppResult<HashMap<Uuid, Vec<PlaylistFolder>>> {
        self.repository
            .playlists_repository()
            .get_playlist_folders()
            .await
    }

    pub async fn get_playlists(&self) -> AppResult<HashMap<Uuid, Vec<Playlist>>> {
        self.repository.playlists_repository().get_playlists().await
    }

    pub async fn get_favorite_playlists(&self) -> AppResult<Vec<Uuid>> {
        self.repository
            .playlists_repository()
            .get_favorite_playlists()
            .await
    }
}
