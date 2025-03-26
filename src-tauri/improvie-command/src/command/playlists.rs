use std::collections::HashMap;

use improvie_logic::{
    AppResult,
    model::{
        items::FolderNode,
        plays::{PlayFolder, PlayFolderNode, Playlist},
    },
};
use uuid::Uuid;

use crate::state::TauriAppState;

#[tauri::command]
pub async fn get_playlist_folders(state: TauriAppState<'_>) -> AppResult<Vec<PlayFolder>> {
    state
        .modules
        .playsts_use_case()
        .get_playlist_folders()
        .await
}

#[tauri::command]
pub async fn get_playlists(state: TauriAppState<'_>) -> AppResult<Vec<Playlist>> {
    state.modules.playsts_use_case().get_playlists().await
}

#[tauri::command]
pub async fn get_favorite_playlists(state: TauriAppState<'_>) -> AppResult<Vec<Uuid>> {
    state
        .modules
        .playsts_use_case()
        .get_favorite_playlists()
        .await
}

#[tauri::command]
pub async fn get_plays_hierarchy_current(
    state: TauriAppState<'_>,
    folder_id: Option<Uuid>,
) -> AppResult<PlayFolderNode> {
    state
        .modules
        .playsts_use_case()
        .get_plays_hierarchy_current(folder_id.unwrap_or(Uuid::nil()))
        .await
}

#[tauri::command]
pub async fn get_plays_hierarchy_loop(
    state: TauriAppState<'_>,
    folder_id: Option<Uuid>,
) -> AppResult<HashMap<Uuid, PlayFolderNode>> {
    state
        .modules
        .playsts_use_case()
        .get_plays_hierarchy_loop(folder_id.unwrap_or(Uuid::nil()))
        .await
}
