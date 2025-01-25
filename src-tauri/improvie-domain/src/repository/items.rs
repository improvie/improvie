use improvie_logic::{
    model::items::{Content, Folder, FolderNode},
    AppResult, Uuid,
};

#[async_trait::async_trait]
pub trait ItemsRepository {
    async fn get_items_hierarchy(&self, folder_id: Uuid) -> AppResult<FolderNode>;

    async fn get_contents(&self) -> AppResult<Vec<Content>>;

    async fn get_folders(&self) -> AppResult<Vec<Folder>>;
}
