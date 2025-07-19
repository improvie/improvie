use std::collections::HashMap;

use improvie_domain::persistence::db::DbTx;
use improvie_domain::{modules::RepositoriesModule, repository::items::ItemsRepository};
use improvie_logic::model::items::{Content, Folder};
use improvie_logic::{DynAppResult, model::items::FolderNode};
use uid::Uid;

use crate::model::items::{
    CreateContentDto, CreateContentResponse, CreateFolderDto, CreateFolderResponse,
};

super::def_use_case!(ItemsUseCase);

impl<R: RepositoriesModule> ItemsUseCase<R> {
    pub async fn get_items_hierarchy_loop(
        &self,
        folder_id: Uid,
    ) -> DynAppResult<HashMap<Uid, FolderNode>> {
        self.repository
            .items_repository()
            .get_items_hierarchy_loop(folder_id)
            .await
    }

    pub async fn get_items_hierarchy_current(&self, folder_id: Uid) -> DynAppResult<FolderNode> {
        self.repository
            .items_repository()
            .get_items_hierarchy_current(folder_id)
            .await
    }

    pub async fn get_contents(&self) -> DynAppResult<Vec<Content>> {
        self.repository.items_repository().get_contents().await
    }

    pub async fn get_folders(&self) -> DynAppResult<Vec<Folder>> {
        self.repository.items_repository().get_folders().await
    }

    pub async fn create_folder(
        &self,
        model: CreateFolderDto,
    ) -> DynAppResult<CreateFolderResponse> {
        let tx = self.repository.begin().await?;
        let conn = tx.connection();

        let parent_folder_id = model.item.parent_folder_id;

        let folder = self
            .repository
            .items_repository()
            .create_folder(conn, model.into())
            .await;
        let folder = super::tx_check!(tx, folder);

        let folder_node = self
            .repository
            .items_repository()
            .get_items_hierarchy_current(parent_folder_id)
            .await;
        let folder_node = super::tx_check!(tx, folder_node);

        tx.commit().await?;

        Ok(CreateFolderResponse {
            folder,
            folder_node,
        })
    }

    pub async fn create_content(
        &self,
        model: CreateContentDto,
    ) -> DynAppResult<CreateContentResponse> {
        let tx = self.repository.begin().await?;
        let conn = tx.connection();

        let parent_folder_id = model.item.parent_folder_id;

        let content = self
            .repository
            .items_repository()
            .create_content(conn, model.into())
            .await;
        let content = super::tx_check!(tx, content);

        let folder_node = self
            .repository
            .items_repository()
            .get_items_hierarchy_current(parent_folder_id)
            .await;
        let folder_node = super::tx_check!(tx, folder_node);

        Ok(CreateContentResponse {
            content,
            folder_node,
        })
    }

    pub async fn delete_item(&self, item_id: Uid) -> DynAppResult<Vec<Uid>> {
        let tx = self.repository.begin().await?;
        let conn = tx.connection();

        let result = self
            .repository
            .items_repository()
            .delete_item(conn, item_id)
            .await;

        super::tx_commit!(tx, result);
    }

    pub async fn update_item_name(&self, item_id: Uid, name: String) -> DynAppResult<()> {
        let tx = self.repository.begin().await?;
        let conn = tx.connection();

        let result = self
            .repository
            .items_repository()
            .update_item_name(conn, item_id, name)
            .await;

        super::tx_commit!(tx, result)
    }
}
