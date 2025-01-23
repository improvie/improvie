use improvie_logic::{model::items::FolderNode, AppResult, Uuid};

#[async_trait::async_trait]
pub trait ItemsRepository {
    async fn get_items_hierarchy(&self, folder_id: Uuid) -> AppResult<FolderNode>;
}
