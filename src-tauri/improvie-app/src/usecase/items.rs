use std::collections::HashMap;

use improvie_domain::{modules::RepositoriesModule, repository::items::ItemsRepository};
use improvie_logic::model::items::{Content, Folder};
use improvie_logic::{AppResult, model::items::FolderNode};
use uuid::Uuid;

use crate::model::items::{
    CreateContentDto, CreateContentResponse, CreateFolderDto, CreateFolderResponse,
};

super::def_use_case!(ItemsUseCase);

impl<R: RepositoriesModule> ItemsUseCase<R> {
    pub async fn get_items_hierarchy_loop(
        &self,
        folder_id: Uuid,
    ) -> AppResult<HashMap<Uuid, FolderNode>> {
        self.repository
            .items_repository()
            .get_items_hierarchy_loop(folder_id)
            .await
    }

    pub async fn get_items_hierarchy_current(&self, folder_id: Uuid) -> AppResult<FolderNode> {
        self.repository
            .items_repository()
            .get_items_hierarchy_current(folder_id)
            .await
    }

    pub async fn get_contents(&self) -> AppResult<Vec<Content>> {
        self.repository.items_repository().get_contents().await
    }

    pub async fn get_folders(&self) -> AppResult<Vec<Folder>> {
        self.repository.items_repository().get_folders().await
    }

    pub async fn create_folder(&self, model: CreateFolderDto) -> AppResult<CreateFolderResponse> {
        let parent_folder_id = model.item.parent_folder_id;

        let folder = self
            .repository
            .items_repository()
            .create_folder(model.into())
            .await?;

        let folder_node = self
            .repository
            .items_repository()
            .get_items_hierarchy_current(parent_folder_id)
            .await?;

        Ok(CreateFolderResponse {
            folder,
            folder_node,
        })
    }

    pub async fn create_content(
        &self,
        model: CreateContentDto,
    ) -> AppResult<CreateContentResponse> {
        let parent_folder_id = model.item.parent_folder_id;

        let content = self
            .repository
            .items_repository()
            .create_content(model.into())
            .await?;

        let folder_node = self
            .repository
            .items_repository()
            .get_items_hierarchy_current(parent_folder_id)
            .await?;

        Ok(CreateContentResponse {
            content,
            folder_node,
        })
    }

    pub async fn delete_item(&self, item_id: Uuid) -> AppResult<()> {
        self.repository.items_repository().delete_item(item_id).await
    }

    pub async fn update_item_name(&self, item_id: Uuid, new_name: String) -> AppResult<()> {
        self.repository
            .items_repository()
            .update_item_name(item_id, new_name)
            .await
    }
}
