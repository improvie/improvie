use std::collections::HashMap;

use improvie_logic::{
    AppResult,
    model::items::{Content, Folder, FolderNode},
};
use uid::Uid;

use crate::model::items::{CreateContentModel, CreateFolderModel};

#[async_trait::async_trait]
pub trait ItemsRepository {
    async fn get_items_hierarchy_current(&self, folder_id: Uid) -> AppResult<FolderNode>;

    async fn get_items_hierarchy_loop(
        &self,
        folder_id: Uid,
    ) -> AppResult<HashMap<Uid, FolderNode>>;

    async fn get_contents(&self) -> AppResult<Vec<Content>>;

    async fn get_folders(&self) -> AppResult<Vec<Folder>>;

    async fn create_folder(&self, model: CreateFolderModel) -> AppResult<Folder>;

    async fn create_content(&self, model: CreateContentModel) -> AppResult<Content>;

    async fn delete_item(&self, item_id: Uid) -> AppResult<Vec<Uid>>;

    async fn update_item_name(&self, item_id: Uid, new_name: String) -> AppResult<()>;
}
