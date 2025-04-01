use improvie_command::command::{dialog, health_check, integration, items, plays, rules};

pub fn generate_handler<R: tauri::Runtime>()
-> impl Fn(tauri::ipc::Invoke<R>) -> bool + Send + Sync + 'static {
    tauri::generate_handler![
        health_check::health_check,
        dialog::file::open_select_content_dialog,
        dialog::file::open_select_thumbnail_dialog,
        // items
        items::get_items_hierarchy,
        items::get_contents,
        items::get_folders,
        items::create_folder,
        items::create_content,
        items::delete_item,
        items::update_item_name,
        // plays
        plays::get_playlists,
        plays::get_play_folders,
        plays::get_favorite_playlists,
        plays::get_plays_hierarchy,
        plays::create_play_folder,
        plays::create_playlist,
        plays::delete_play_item,
        plays::update_play_item_name,
        // rules
        rules::get_rules,
        rules::update_rules,
        rules::get_rules_format,
        // integration
        integration::import_youtube_video,
    ]
}
