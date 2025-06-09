use std::collections::HashMap;

use improvie_domain::{modules::RepositoriesModule, repository::plays::PlaystsRepository};
use improvie_logic::{
    DynAppResult,
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
            .get_plays_hierarchy_current(folder_id)
            .await
    }

    pub async fn get_plays_hierarchy_loop(
        &self,
        folder_id: Uid,
    ) -> DynAppResult<HashMap<Uid, PlayFolderNode>> {
        self.repository
            .playsts_repository()
            .get_plays_hierarchy_loop(folder_id)
            .await
    }

    pub async fn get_play_folders(&self) -> DynAppResult<Vec<PlayFolder>> {
        self.repository
            .playsts_repository()
            .get_play_folders()
            .await
    }

    pub async fn get_playlists(&self) -> DynAppResult<Vec<Playlist>> {
        self.repository.playsts_repository().get_playlists().await
    }

    pub async fn get_favorite_playlists(&self) -> DynAppResult<Vec<Uid>> {
        self.repository
            .playsts_repository()
            .get_favorite_playlists()
            .await
    }

    pub async fn add_favorite_playlist(&self, playlist_id: Uid) -> DynAppResult<()> {
        self.repository
            .playsts_repository()
            .add_favorite_playlist(playlist_id)
            .await
    }

    pub async fn remove_favorite_playlist(&self, playlist_id: Uid) -> DynAppResult<()> {
        self.repository
            .playsts_repository()
            .remove_favorite_playlist(playlist_id)
            .await
    }

    pub async fn create_play_folder(
        &self,
        model: CreatePlayFolderDto,
    ) -> DynAppResult<CreatePlayFolderResponse> {
        let parent_folder_id = model.item.parent_folder_id;

        let folder = self
            .repository
            .playsts_repository()
            .create_play_folder(model.into())
            .await?;

        let folder_node = self
            .repository
            .playsts_repository()
            .get_plays_hierarchy_current(parent_folder_id)
            .await?;

        Ok(CreatePlayFolderResponse {
            folder,
            folder_node,
        })
    }

    pub async fn create_playlist(
        &self,
        model: CreatePlaylistDto,
    ) -> DynAppResult<CreatePlaylistResponse> {
        let parent_folder_id = model.item.parent_folder_id;

        let playlist = self
            .repository
            .playsts_repository()
            .create_playlist(model.into())
            .await?;

        let folder_node = self
            .repository
            .playsts_repository()
            .get_plays_hierarchy_current(parent_folder_id)
            .await?;

        Ok(CreatePlaylistResponse {
            playlist,
            folder_node,
        })
    }

    pub async fn delete_play_item(&self, play_id: Uid) -> DynAppResult<Vec<Uid>> {
        self.repository
            .playsts_repository()
            .delete_play_item(play_id)
            .await
    }

    pub async fn update_play_item_name(&self, play_id: Uid, name: String) -> DynAppResult<()> {
        self.repository
            .playsts_repository()
            .update_play_item_name(play_id, name)
            .await
    }
}
