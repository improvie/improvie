use std::collections::HashMap;

use improvie_app::model::items::{CreateContentResponse, CreateFolderDto, CreateFolderResponse};
use improvie_logic::{
    DynAppResult,
    model::items::{Content, Folder, FolderNode},
};
use uid::Uid;

use crate::{
    model::{items::CreateContentRequest, yt::YtErrorWrapper},
    state::TauriAppState,
};

#[tauri::command]
pub async fn get_items_hierarchy(
    state: TauriAppState<'_>,
    folder_id: Uid,
) -> DynAppResult<HashMap<Uid, FolderNode>> {
    state
        .modules
        .items_use_case()
        .get_items_hierarchy_loop(folder_id)
        .await
}

#[tauri::command]
pub async fn get_contents(state: TauriAppState<'_>) -> DynAppResult<Vec<Content>> {
    state.modules.items_use_case().get_contents().await
}

#[tauri::command]
pub async fn get_folders(state: TauriAppState<'_>) -> DynAppResult<Vec<Folder>> {
    state.modules.items_use_case().get_folders().await
}

#[tauri::command]
pub async fn create_folder(
    state: TauriAppState<'_>,
    request: CreateFolderDto,
) -> DynAppResult<CreateFolderResponse> {
    state.modules.items_use_case().create_folder(request).await
}

#[tauri::command]
pub async fn create_content(
    state: TauriAppState<'_>,
    request: CreateContentRequest,
) -> DynAppResult<CreateContentResponse> {
    let seconds = youtube::get_duration(request.content_path.clone()).map_err(YtErrorWrapper)?;
    state
        .modules
        .items_use_case()
        .create_content(request.into_dto(seconds))
        .await
}

#[tauri::command]
pub async fn delete_item(state: TauriAppState<'_>, item_id: Uid) -> DynAppResult<Vec<Uid>> {
    state.modules.items_use_case().delete_item(item_id).await
}

#[tauri::command]
pub async fn update_item_name(
    state: TauriAppState<'_>,
    item_id: Uid,
    name: String,
) -> DynAppResult<()> {
    state
        .modules
        .items_use_case()
        .update_item_name(item_id, name)
        .await
}
