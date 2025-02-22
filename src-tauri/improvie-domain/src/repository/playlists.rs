use std::collections::HashMap;

use improvie_logic::{
    AppResult, Uuid,
    model::playlist::{Playlist, PlaylistFolder},
};

#[async_trait::async_trait]
pub trait PlaylistsRepository {
    async fn get_playlist_folders(&self) -> AppResult<HashMap<Uuid, Vec<PlaylistFolder>>>;

    async fn get_playlists(&self) -> AppResult<HashMap<Uuid, Vec<Playlist>>>;

    async fn get_favorite_playlists(&self) -> AppResult<Vec<Uuid>>;
}
