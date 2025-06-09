use std::collections::HashMap;

use improvie_logic::{
    DynAppResult,
    model::plays::{PlayFolder, PlayFolderNode, Playlist},
};
use uid::Uid;

use crate::model::plays::{CreatePlayFolderModel, CreatePlaylistModel};

#[async_trait::async_trait]
pub trait PlaystsRepository {
    type DbConnection<'a>: crate::persistence::db::DbConnection<'a>;

    async fn get_plays_hierarchy_current(&self, folder_id: Uid) -> DynAppResult<PlayFolderNode>;

    async fn get_plays_hierarchy_loop(
        &self,
        folder_id: Uid,
    ) -> DynAppResult<HashMap<Uid, PlayFolderNode>>;

    async fn get_play_folders(&self) -> DynAppResult<Vec<PlayFolder>>;

    async fn get_playlists(&self) -> DynAppResult<Vec<Playlist>>;

    async fn get_favorite_playlists(&self) -> DynAppResult<Vec<Uid>>;

    async fn add_favorite_playlist(&self, playlist_id: Uid) -> DynAppResult<()>;

    async fn remove_favorite_playlist(&self, playlist_id: Uid) -> DynAppResult<()>;

    async fn create_play_folder(&self, model: CreatePlayFolderModel) -> DynAppResult<PlayFolder>;

    async fn create_playlist(&self, model: CreatePlaylistModel) -> DynAppResult<Playlist>;

    async fn delete_play_item(&self, play_id: Uid) -> DynAppResult<Vec<Uid>>;

    async fn update_play_item_name(&self, play_id: Uid, name: String) -> DynAppResult<()>;
}
