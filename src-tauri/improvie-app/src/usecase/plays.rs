use std::collections::HashMap;

use improvie_domain::{
    modules::RepositoriesModule, persistence::db::DbTx, repository::plays::PlaystsRepository,
};
use improvie_logic::{
    DynAppResult,
    constant::plays::PlayItemKind,
    model::plays::{PlayFolder, PlayFolderNode, Playlist},
};
use uid::Uid;

use crate::model::plays::{
    CreatePlayFolderDto, CreatePlayFolderResponse, CreatePlaylistDto, CreatePlaylistResponse,
};

super::def_use_case!(PlaysUseCase);

impl<R: RepositoriesModule> PlaysUseCase<R> {
    pub async fn get_plays_hierarchy_current(
        &self,
        folder_id: Uid,
    ) -> DynAppResult<PlayFolderNode> {
        self.repository
            .playsts_repository()
            .get_plays_hierarchy_current(self.repository.connection(), folder_id)
            .await
    }

    pub async fn get_plays_hierarchy_loop(
        &self,
        folder_id: Uid,
    ) -> DynAppResult<HashMap<Uid, PlayFolderNode>> {
        self.repository
            .playsts_repository()
            .get_plays_hierarchy_loop(self.repository.connection(), folder_id)
            .await
    }

    pub async fn get_play_folders(&self) -> DynAppResult<Vec<PlayFolder>> {
        self.repository
            .playsts_repository()
            .get_play_folders(self.repository.connection())
            .await
    }

    pub async fn get_playlists(&self) -> DynAppResult<Vec<Playlist>> {
        self.repository
            .playsts_repository()
            .get_playlists(self.repository.connection())
            .await
    }

    pub async fn get_favorite_playlists(&self) -> DynAppResult<Vec<Uid>> {
        self.repository
            .playsts_repository()
            .get_favorite_playlists(self.repository.connection())
            .await
    }

    pub async fn add_favorite_playlist(&self, playlist_id: Uid) -> DynAppResult<()> {
        let tx = self.repository.begin().await?;
        let conn = tx.connection();

        let result = self
            .repository
            .playsts_repository()
            .add_favorite_playlist(conn, playlist_id)
            .await;

        super::tx_commit!(tx, result)
    }

    pub async fn remove_favorite_playlist(&self, playlist_id: Uid) -> DynAppResult<()> {
        let tx = self.repository.begin().await?;
        let conn = tx.connection();
        let result = self
            .repository
            .playsts_repository()
            .remove_favorite_playlist(conn, playlist_id)
            .await;

        super::tx_commit!(tx, result)
    }

    pub async fn create_play_folder(
        &self,
        model: CreatePlayFolderDto,
    ) -> DynAppResult<CreatePlayFolderResponse> {
        let tx = self.repository.begin().await?;
        let conn = tx.connection();

        let parent_folder_id = model.item.parent_folder_id;

        let folder = self
            .repository
            .playsts_repository()
            .create_play_folder(conn, model.into())
            .await;
        let folder = super::tx_check!(tx, folder);

        let folder_node = self
            .repository
            .playsts_repository()
            .get_plays_hierarchy_current(conn, parent_folder_id)
            .await;
        let folder_node = super::tx_check!(tx, folder_node);

        tx.commit().await?;

        Ok(CreatePlayFolderResponse {
            folder,
            folder_node,
        })
    }

    pub async fn create_playlist(
        &self,
        model: CreatePlaylistDto,
    ) -> DynAppResult<CreatePlaylistResponse> {
        let tx = self.repository.begin().await?;
        let conn = tx.connection();

        let parent_folder_id = model.item.parent_folder_id;

        let playlist = self
            .repository
            .playsts_repository()
            .create_playlist(conn, model.into())
            .await;
        let playlist = super::tx_check!(tx, playlist);

        let folder_node = self
            .repository
            .playsts_repository()
            .get_plays_hierarchy_current(conn, parent_folder_id)
            .await;
        let folder_node = super::tx_check!(tx, folder_node);

        tx.commit().await?;

        Ok(CreatePlaylistResponse {
            playlist,
            folder_node,
        })
    }

    pub async fn delete_play_items(
        &self,
        play_ids: Vec<Uid>,
    ) -> DynAppResult<Vec<(Uid, PlayItemKind)>> {
        let tx = self.repository.begin().await?;
        let conn = tx.connection();

        let result = self
            .repository
            .playsts_repository()
            .delete_play_items(conn, play_ids)
            .await;

        super::tx_commit!(tx, result)
    }

    pub async fn update_play_item_name(&self, play_id: Uid, name: String) -> DynAppResult<()> {
        let tx = self.repository.begin().await?;
        let conn = tx.connection();

        let result = self
            .repository
            .playsts_repository()
            .update_play_item_name(conn, play_id, name)
            .await;

        super::tx_commit!(tx, result)
    }
}
