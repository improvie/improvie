use std::collections::HashMap;

use improvie_logic::{
    DynAppResult,
    constant::plays::PlayItemKind,
    model::plays::{PlayFolder, PlayFolderNode, Playlist},
};
use uid::Uid;

use crate::model::plays::{CreatePlayFolderModel, CreatePlaylistModel};

#[async_trait::async_trait]
pub trait PlaystsRepository {
    type DbConnection<'a>: crate::persistence::db::DbConnection<'a>;

    async fn get_plays_hierarchy_current(
        &self,
        conn: Self::DbConnection<'_>,
        folder_id: Uid,
    ) -> DynAppResult<PlayFolderNode>;

    async fn get_plays_hierarchy_loop(
        &self,
        conn: Self::DbConnection<'_>,
        folder_id: Uid,
    ) -> DynAppResult<HashMap<Uid, PlayFolderNode>>;

    async fn get_play_folders(&self, conn: Self::DbConnection<'_>)
    -> DynAppResult<Vec<PlayFolder>>;

    async fn get_playlists(&self, conn: Self::DbConnection<'_>) -> DynAppResult<Vec<Playlist>>;

    async fn get_favorite_playlists(&self, conn: Self::DbConnection<'_>) -> DynAppResult<Vec<Uid>>;

    async fn add_favorite_playlist(
        &self,
        conn: Self::DbConnection<'_>,
        playlist_id: Uid,
    ) -> DynAppResult<()>;

    async fn remove_favorite_playlist(
        &self,
        conn: Self::DbConnection<'_>,
        playlist_id: Uid,
    ) -> DynAppResult<()>;

    async fn create_play_folder(
        &self,
        conn: Self::DbConnection<'_>,
        model: CreatePlayFolderModel,
    ) -> DynAppResult<PlayFolder>;

    async fn create_playlist(
        &self,
        conn: Self::DbConnection<'_>,
        model: CreatePlaylistModel,
    ) -> DynAppResult<Playlist>;

    async fn delete_play_items(
        &self,
        conn: Self::DbConnection<'_>,
        play_ids: Vec<Uid>,
    ) -> DynAppResult<Vec<(Uid, PlayItemKind)>>;

    async fn update_play_item_name(
        &self,
        conn: Self::DbConnection<'_>,
        play_id: Uid,
        name: String,
    ) -> DynAppResult<()>;
}
