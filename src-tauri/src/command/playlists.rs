use improvie_logic::{
    model::playlist::{Play, Playlist},
    AppResult, Uuid,
};
use tauri::State;

use crate::modules::Modules;

#[tauri::command]
pub async fn get_playlists(modules: State<'_, Modules>) -> AppResult<Vec<Playlist>> {
    modules.playlists_use_case().get_playlists().await
}

#[tauri::command]
pub async fn get_plays(modules: State<'_, Modules>) -> AppResult<Vec<Play>> {
    modules.playlists_use_case().get_plays().await
}

#[tauri::command]
pub async fn get_favorite_playlists(modules: State<'_, Modules>) -> AppResult<Vec<Uuid>> {
    modules.playlists_use_case().get_favorite_playlists().await
}
