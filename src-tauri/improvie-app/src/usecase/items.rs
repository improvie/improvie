use improvie_domain::{modules::RepositoriesModule, repository::items::ItemsRepository};
use improvie_logic::{model::items::FolderNode, AppResult, Uuid};

pub struct ItemsUseCase<R: RepositoriesModule> {
    repository: R,
}

impl<R: RepositoriesModule> ItemsUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn get_items_hierarchy(&self, folder_id: Uuid) -> AppResult<FolderNode> {
        self.repository
            .items_repository()
            .get_items_hierarchy(folder_id)
            .await
    }
}
