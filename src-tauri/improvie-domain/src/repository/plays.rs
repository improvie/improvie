use std::collections::HashMap;

use improvie_logic::{
    AppResult,
    model::plays::{PlayFolder, PlayFolderNode, Playlist},
};
use uid::Uid;

use crate::model::plays::{CreatePlayFolderModel, CreatePlaylistModel};

#[async_trait::async_trait]
pub trait PlaystsRepository {
    async fn get_plays_hierarchy_current(&self, folder_id: Uid) -> AppResult<PlayFolderNode>;

    async fn get_plays_hierarchy_loop(
        &self,
        folder_id: Uid,
    ) -> AppResult<HashMap<Uid, PlayFolderNode>>;

    async fn get_play_folders(&self) -> AppResult<Vec<PlayFolder>>;

    async fn get_playlists(&self) -> AppResult<Vec<Playlist>>;

    async fn get_favorite_playlists(&self) -> AppResult<Vec<Uid>>;

    async fn create_play_folder(&self, model: CreatePlayFolderModel) -> AppResult<PlayFolder>;

    async fn create_playlist(&self, model: CreatePlaylistModel) -> AppResult<Playlist>;

    async fn delete_play_item(&self, play_id: Uid) -> AppResult<Vec<Uid>>;

    async fn update_play_item_name(&self, play_id: Uid, name: String) -> AppResult<()>;
}
