use improvie_command::command::{dialog, health_check, items, plays, rules};

pub fn generate_handler<R: tauri::Runtime>()
-> impl Fn(tauri::ipc::Invoke<R>) -> bool + Send + Sync + 'static {
    tauri::generate_handler![
        health_check::health_check,
        dialog::file::open_select_content_dialog,
        dialog::file::open_select_thumbnail_dialog,
        items::get_items_hierarchy,
        items::get_contents,
        items::get_folders,
        items::create_folder,
        items::create_content,
        plays::get_playlists,
        plays::get_play_folders,
        plays::get_favorite_playlists,
        plays::get_plays_hierarchy,
        plays::create_play_folder,
        plays::create_playlist,
        rules::get_rules,
        rules::update_rules,
        rules::get_rules_format,
    ]
}
