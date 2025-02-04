use std::collections::HashMap;

use improvie_logic::{
    model::playlist::{Playlist, PlaylistFolder},
    AppResult, Uuid,
};
use tauri::State;

use crate::modules::Modules;

#[tauri::command]
pub async fn get_playlist_folders(
    modules: State<'_, Modules>,
) -> AppResult<HashMap<Uuid, Vec<PlaylistFolder>>> {
    modules.playlists_use_case().get_playlist_folders().await
}

#[tauri::command]
pub async fn get_playlists(modules: State<'_, Modules>) -> AppResult<HashMap<Uuid, Vec<Playlist>>> {
    modules.playlists_use_case().get_playlists().await
}

#[tauri::command]
pub async fn get_favorite_playlists(modules: State<'_, Modules>) -> AppResult<Vec<Uuid>> {
    modules.playlists_use_case().get_favorite_playlists().await
}
