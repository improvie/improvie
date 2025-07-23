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

    async fn get_items_hierarchy_current(
        &self,
        conn: Self::DbConnection<'_>,
        folder_id: Uid,
    ) -> DynAppResult<FolderNode>;

    async fn get_items_hierarchy_loop(
        &self,
        conn: Self::DbConnection<'_>,
        folder_id: Uid,
    ) -> DynAppResult<HashMap<Uid, FolderNode>>;

    async fn get_contents(&self, conn: Self::DbConnection<'_>) -> DynAppResult<Vec<Content>>;

    async fn get_content_by_id(
        &self,
        conn: Self::DbConnection<'_>,
        uid: Uid,
    ) -> DynAppResult<Option<Content>>;

    async fn get_folders(&self, conn: Self::DbConnection<'_>) -> DynAppResult<Vec<Folder>>;

    async fn get_folder_by_id(
        &self,
        conn: Self::DbConnection<'_>,
        uid: Uid,
    ) -> DynAppResult<Option<Folder>>;

    async fn create_folder(
        &self,
        conn: Self::DbConnection<'_>,
        model: CreateFolderModel,
    ) -> DynAppResult<Folder>;

    async fn create_content(
        &self,
        conn: Self::DbConnection<'_>,
        model: CreateContentModel,
    ) -> DynAppResult<Content>;

    async fn delete_item(
        &self,
        conn: Self::DbConnection<'_>,
        item_id: Uid,
    ) -> DynAppResult<Vec<Uid>>;

    async fn update_item_name(
        &self,
        conn: Self::DbConnection<'_>,
        item_id: Uid,
        new_name: String,
    ) -> DynAppResult<()>;
}
