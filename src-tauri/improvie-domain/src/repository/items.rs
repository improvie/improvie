use std::collections::HashMap;

use improvie_logic::{
    DynAppResult,
    model::items::{Content, Folder, FolderNode},
};
use uid::Uid;

use crate::model::items::{CreateContentModel, CreateFolderModel};

#[async_trait::async_trait]
pub trait ItemsRepository {
    type DbConnection<'a>: crate::persistence::db::DbConnection<'a>;

    async fn get_items_hierarchy_current(&self, folder_id: Uid) -> DynAppResult<FolderNode>;

    async fn get_items_hierarchy_loop(
        &self,
        folder_id: Uid,
    ) -> DynAppResult<HashMap<Uid, FolderNode>>;

    async fn get_contents(&self) -> DynAppResult<Vec<Content>>;

    async fn get_folders(&self) -> DynAppResult<Vec<Folder>>;

    async fn create_folder(&self, model: CreateFolderModel) -> DynAppResult<Folder>;

    async fn create_content(&self, model: CreateContentModel) -> DynAppResult<Content>;

    async fn delete_item(&self, item_id: Uid) -> DynAppResult<Vec<Uid>>;

    async fn update_item_name(&self, item_id: Uid, new_name: String) -> DynAppResult<()>;
}
