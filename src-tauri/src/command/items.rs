use improvie_logic::{model::items::FolderNode, AppResult, Uuid};
use tauri::State;

use crate::modules::Modules;

#[tauri::command]
pub async fn get_items_hierarchy(
    modules: State<'_, Modules>,
    folder_id: Option<Uuid>,
) -> AppResult<FolderNode> {
    modules
        .items_use_case()
        .get_items_hierarchy(folder_id.unwrap_or(Uuid::nil()))
        .await
}
