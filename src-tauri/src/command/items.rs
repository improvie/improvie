use std::collections::HashMap;

use improvie_app::model::items::{CreateContentDto, CreateFolderDto};
use improvie_logic::{
    model::items::{Content, Folder, FolderNode},
    AppResult, Uuid,
};
use tauri::State;

use crate::modules::Modules;

#[tauri::command]
pub async fn get_items_hierarchy(
    modules: State<'_, Modules>,
    folder_id: Option<Uuid>,
) -> AppResult<HashMap<Uuid, FolderNode>> {
    modules
        .items_use_case()
        .get_items_hierarchy(folder_id.unwrap_or(Uuid::nil()))
        .await
}

#[tauri::command]
pub async fn get_contents(modules: State<'_, Modules>) -> AppResult<Vec<Content>> {
    modules.items_use_case().get_contents().await
}

#[tauri::command]
pub async fn get_folders(modules: State<'_, Modules>) -> AppResult<Vec<Folder>> {
    modules.items_use_case().get_folders().await
}

#[tauri::command]
pub async fn create_folder(modules: State<'_, Modules>, dto: CreateFolderDto) -> AppResult<Folder> {
    modules.items_use_case().create_folder(dto).await
}

#[tauri::command]
pub async fn create_content(
    modules: State<'_, Modules>,
    dto: CreateContentDto,
) -> AppResult<Content> {
    modules.items_use_case().create_content(dto).await
}
