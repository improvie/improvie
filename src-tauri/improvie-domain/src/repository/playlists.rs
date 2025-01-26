use improvie_logic::{
    model::playlist::{Play, Playlist},
    AppResult, Uuid,
};

#[async_trait::async_trait]
pub trait PlaylistsRepository {
    async fn get_playlists(&self) -> AppResult<Vec<Playlist>>;

    async fn get_plays(&self) -> AppResult<Vec<Play>>;

    async fn get_favorite_playlists(&self) -> AppResult<Vec<Uuid>>;
}
