use std::collections::HashMap;

use improvie_app::model::items::{
    CreateContentDto, CreateContentResponse, CreateFolderDto, CreateFolderResponse,
};
use improvie_logic::{
    AppResult,
    model::items::{Content, Folder, FolderNode},
};
use uuid::Uuid;

use crate::state::TauriAppState;

#[tauri::command]
pub async fn get_items_hierarchy(
    state: TauriAppState<'_>,
    folder_id: Option<Uuid>,
) -> AppResult<HashMap<Uuid, FolderNode>> {
    state
        .modules
        .items_use_case()
        .get_items_hierarchy_loop(folder_id.unwrap_or(Uuid::nil()))
        .await
}

#[tauri::command]
pub async fn get_contents(state: TauriAppState<'_>) -> AppResult<Vec<Content>> {
    state.modules.items_use_case().get_contents().await
}

#[tauri::command]
pub async fn get_folders(state: TauriAppState<'_>) -> AppResult<Vec<Folder>> {
    state.modules.items_use_case().get_folders().await
}

#[tauri::command]
pub async fn create_folder(
    state: TauriAppState<'_>,
    dto: CreateFolderDto,
) -> AppResult<CreateFolderResponse> {
    state.modules.items_use_case().create_folder(dto).await
}

#[tauri::command]
pub async fn create_content(
    state: TauriAppState<'_>,
    dto: CreateContentDto,
) -> AppResult<CreateContentResponse> {
    state.modules.items_use_case().create_content(dto).await
}

#[tauri::command]
pub async fn delete_item(state: TauriAppState<'_>, item_id: Uuid) -> AppResult<Vec<Uuid>> {
    state.modules.items_use_case().delete_item(item_id).await
}

#[tauri::command]
pub async fn update_item_name(
    state: TauriAppState<'_>,
    item_id: Uuid,
    name: String,
) -> AppResult<()> {
    state
        .modules
        .items_use_case()
        .update_item_name(item_id, name)
        .await
}
