use std::collections::HashMap;

use improvie_logic::{
    AppResult, Uuid,
    model::playlist::{Playlist, PlaylistFolder},
};

use crate::state::TauriAppState;

#[tauri::command]
pub async fn get_playlist_folders(
    state: TauriAppState<'_>,
) -> AppResult<HashMap<Uuid, Vec<PlaylistFolder>>> {
    state
        .modules
        .playlists_use_case()
        .get_playlist_folders()
        .await
}

#[tauri::command]
pub async fn get_playlists(state: TauriAppState<'_>) -> AppResult<HashMap<Uuid, Vec<Playlist>>> {
    state.modules.playlists_use_case().get_playlists().await
}

#[tauri::command]
pub async fn get_favorite_playlists(state: TauriAppState<'_>) -> AppResult<Vec<Uuid>> {
    state
        .modules
        .playlists_use_case()
        .get_favorite_playlists()
        .await
}
