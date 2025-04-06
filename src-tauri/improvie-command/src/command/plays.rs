use std::collections::HashMap;

use improvie_app::model::plays::{
    CreatePlayFolderDto, CreatePlayFolderResponse, CreatePlaylistDto, CreatePlaylistResponse,
};
use improvie_logic::{
    AppResult,
    model::plays::{PlayFolder, PlayFolderNode, Playlist},
};
use uid::Uid;

use crate::state::TauriAppState;

#[tauri::command]
pub async fn get_play_folders(state: TauriAppState<'_>) -> AppResult<Vec<PlayFolder>> {
    state.modules.plays_use_case().get_play_folders().await
}

#[tauri::command]
pub async fn get_playlists(state: TauriAppState<'_>) -> AppResult<Vec<Playlist>> {
    state.modules.plays_use_case().get_playlists().await
}

#[tauri::command]
pub async fn get_favorite_playlists(state: TauriAppState<'_>) -> AppResult<Vec<Uid>> {
    state
        .modules
        .plays_use_case()
        .get_favorite_playlists()
        .await
}

#[tauri::command]
pub async fn add_favorite_playlist(state: TauriAppState<'_>, id: Uid) -> AppResult<()> {
    state
        .modules
        .plays_use_case()
        .add_favorite_playlist(id)
        .await
}

#[tauri::command]
pub async fn remove_favorite_playlist(state: TauriAppState<'_>, id: Uid) -> AppResult<()> {
    state
        .modules
        .plays_use_case()
        .remove_favorite_playlist(id)
        .await
}

#[tauri::command]
pub async fn get_plays_hierarchy(
    state: TauriAppState<'_>,
    folder_id: Option<Uid>,
) -> AppResult<HashMap<Uid, PlayFolderNode>> {
    state
        .modules
        .plays_use_case()
        .get_plays_hierarchy_loop(folder_id.unwrap_or(Uid::nil()))
        .await
}

#[tauri::command]
pub async fn create_play_folder(
    state: TauriAppState<'_>,
    dto: CreatePlayFolderDto,
) -> AppResult<CreatePlayFolderResponse> {
    state.modules.plays_use_case().create_play_folder(dto).await
}

#[tauri::command]
pub async fn create_playlist(
    state: TauriAppState<'_>,
    dto: CreatePlaylistDto,
) -> AppResult<CreatePlaylistResponse> {
    state.modules.plays_use_case().create_playlist(dto).await
}

#[tauri::command]
pub async fn delete_play_item(state: TauriAppState<'_>, play_id: Uid) -> AppResult<Vec<Uid>> {
    state
        .modules
        .plays_use_case()
        .delete_play_item(play_id)
        .await
}

#[tauri::command]
pub async fn update_play_item_name(
    state: TauriAppState<'_>,
    play_id: Uid,
    name: String,
) -> AppResult<()> {
    state
        .modules
        .plays_use_case()
        .update_play_item_name(play_id, name)
        .await
}
