use std::collections::HashMap;

use improvie_domain::{modules::RepositoriesModule, repository::plays::PlaystsRepository};
use improvie_logic::{
    AppResult,
    model::plays::{PlayFolder, PlayFolderNode, Playlist},
};
use uuid::Uuid;

super::def_use_case!(PlaystsUseCase);

impl<R: RepositoriesModule> PlaystsUseCase<R> {
    pub async fn get_plays_hierarchy_current(&self, folder_id: Uuid) -> AppResult<PlayFolderNode> {
        self.repository
            .playsts_repository()
            .get_plays_hierarchy_current(folder_id)
            .await
    }

    pub async fn get_plays_hierarchy_loop(
        &self,
        folder_id: Uuid,
    ) -> AppResult<HashMap<Uuid, PlayFolderNode>> {
        self.repository
            .playsts_repository()
            .get_plays_hierarchy_loop(folder_id)
            .await
    }

    pub async fn get_play_folders(&self) -> AppResult<Vec<PlayFolder>> {
        self.repository
            .playsts_repository()
            .get_play_folders()
            .await
    }

    pub async fn get_playlists(&self) -> AppResult<Vec<Playlist>> {
        self.repository.playsts_repository().get_playlists().await
    }

    pub async fn get_favorite_playlists(&self) -> AppResult<Vec<Uuid>> {
        self.repository
            .playsts_repository()
            .get_favorite_playlists()
            .await
    }
}
