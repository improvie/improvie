use std::collections::HashMap;

use improvie_logic::{
    AppResult,
    model::plays::{PlayFolder, PlayFolderNode, Playlist},
};
use uuid::Uuid;

#[async_trait::async_trait]
pub trait PlaystsRepository {
    async fn get_plays_hierarchy_current(&self, folder_id: Uuid) -> AppResult<PlayFolderNode>;

    async fn get_plays_hierarchy_loop(
        &self,
        folder_id: Uuid,
    ) -> AppResult<HashMap<Uuid, PlayFolderNode>>;

    async fn get_play_folders(&self) -> AppResult<Vec<PlayFolder>>;

    async fn get_playlists(&self) -> AppResult<Vec<Playlist>>;

    async fn get_favorite_playlists(&self) -> AppResult<Vec<Uuid>>;
}
