use std::collections::HashMap;

use improvie_logic::{
    model::items::{Content, Folder, FolderNode},
    AppResult, Uuid,
};

use crate::model::items::{CreateContentModel, CreateFolderModel};

#[async_trait::async_trait]
pub trait ItemsRepository {
    async fn get_items_hierarchy(&self, folder_id: Uuid) -> AppResult<HashMap<Uuid, FolderNode>>;

    async fn get_contents(&self) -> AppResult<Vec<Content>>;

    async fn get_folders(&self) -> AppResult<Vec<Folder>>;

    async fn create_folder(&self, model: CreateFolderModel) -> AppResult<Folder>;

    async fn create_content(&self, model: CreateContentModel) -> AppResult<Content>;
}
