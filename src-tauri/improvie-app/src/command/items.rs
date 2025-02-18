use std::collections::HashMap;

use crate::{
    model::items::{CreateContentDto, CreateContentResponse, CreateFolderDto},
    state::TauriAppState,
};
use improvie_logic::{
    model::items::{Content, Folder, FolderNode},
    AppResult, Uuid,
};

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
pub async fn create_folder(state: TauriAppState<'_>, dto: CreateFolderDto) -> AppResult<Folder> {
    state.modules.items_use_case().create_folder(dto).await
}

#[tauri::command]
pub async fn create_content(
    state: TauriAppState<'_>,
    dto: CreateContentDto,
) -> AppResult<CreateContentResponse> {
    state.modules.items_use_case().create_content(dto).await
}
