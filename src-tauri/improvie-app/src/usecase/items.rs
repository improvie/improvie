use std::collections::HashMap;

use improvie_domain::{modules::RepositoriesModule, repository::items::ItemsRepository};
use improvie_logic::model::items::{Content, Folder};
use improvie_logic::{model::items::FolderNode, AppResult, Uuid};

use crate::model::items::{CreateContentDto, CreateFolderDto};

pub struct ItemsUseCase<R: RepositoriesModule> {
    repository: R,
}

impl<R: RepositoriesModule> ItemsUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn get_items_hierarchy(
        &self,
        folder_id: Uuid,
    ) -> AppResult<HashMap<Uuid, FolderNode>> {
        self.repository
            .items_repository()
            .get_items_hierarchy(folder_id)
            .await
    }

    pub async fn get_contents(&self) -> AppResult<Vec<Content>> {
        self.repository.items_repository().get_contents().await
    }

    pub async fn get_folders(&self) -> AppResult<Vec<Folder>> {
        self.repository.items_repository().get_folders().await
    }

    pub async fn create_folder(&self, model: CreateFolderDto) -> AppResult<Folder> {
        self.repository
            .items_repository()
            .create_folder(model.into())
            .await
    }

    pub async fn create_content(&self, model: CreateContentDto) -> AppResult<Content> {
        self.repository
            .items_repository()
            .create_content(model.into())
            .await
    }
}
