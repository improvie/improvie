use crate::state::TauriAppState;

#[tauri::command]
pub async fn update_content_by_used(
    state: TauriAppState<'_>,
    content_id: uid::Uid,
) -> improvie_logic::DynAppResult<()> {
    state
        .modules
        .recents_use_case()
        .update_content(content_id)
        .await
}

#[tauri::command]
pub async fn update_playlist_by_used(
    state: TauriAppState<'_>,
    playlist_id: uid::Uid,
) -> improvie_logic::DynAppResult<()> {
    state
        .modules
        .recents_use_case()
        .update_playlist(playlist_id)
        .await
}

#[tauri::command]
pub async fn get_recent_contents(
    state: TauriAppState<'_>,
    limit: Option<usize>,
) -> improvie_logic::DynAppResult<Vec<uid::Uid>> {
    state
        .modules
        .recents_use_case()
        .get_recent_contents(limit)
        .await
}

#[tauri::command]
pub async fn get_recent_playlists(
    state: TauriAppState<'_>,
    limit: Option<usize>,
) -> improvie_logic::DynAppResult<Vec<uid::Uid>> {
    state
        .modules
        .recents_use_case()
        .get_recent_playlists(limit)
        .await
}
