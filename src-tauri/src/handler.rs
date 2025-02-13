use improvie_app::command::{health_check, items, playlists};

pub fn generate_handler<R: tauri::Runtime>(
) -> impl Fn(tauri::ipc::Invoke<R>) -> bool + Send + Sync + 'static {
    tauri::generate_handler![
        health_check::health_check,
        items::get_items_hierarchy,
        items::get_contents,
        items::get_folders,
        items::create_folder,
        items::create_content,
        playlists::get_playlists,
        playlists::get_playlist_folders,
        playlists::get_favorite_playlists,
    ]
}
