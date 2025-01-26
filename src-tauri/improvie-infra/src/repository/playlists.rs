use improvie_domain::repository::playlists::PlaylistsRepository;
use improvie_logic::{
    model::playlist::{Play, Playlist},
    AppResult, Uuid,
};

use super::def_repository_impl;

def_repository_impl!(PlaylistsRepositoryImpl);

#[async_trait::async_trait]
impl PlaylistsRepository for PlaylistsRepositoryImpl {
    async fn get_playlists(&self) -> AppResult<Vec<Playlist>> {
        todo!()
    }

    async fn get_plays(&self) -> AppResult<Vec<Play>> {
        todo!()
    }

    async fn get_favorite_playlists(&self) -> AppResult<Vec<Uuid>> {
        todo!()
    }
}
