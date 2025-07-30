use improvie_command::command::{dialog, items, plays, recents, rules, settings, youtube};

pub fn generate_handler<R: tauri::Runtime>()
-> impl Fn(tauri::ipc::Invoke<R>) -> bool + Send + Sync + 'static {
    tauri::generate_handler![
        // settings
        settings::get_app_settings,
        settings::set_app_settings,
        // dialog
        dialog::file::open_select_content_dialog,
        dialog::file::open_select_thumbnail_dialog,
        // items
        items::get_items_hierarchy,
        items::get_contents,
        items::get_folders,
        items::create_folder,
        items::create_content,
        items::delete_items,
        items::update_item_name,
        // plays
        plays::get_playlists,
        plays::get_play_folders,
        plays::get_favorite_playlists,
        plays::add_favorite_playlist,
        plays::remove_favorite_playlist,
        plays::get_plays_hierarchy,
        plays::create_play_folder,
        plays::create_playlist,
        plays::delete_play_items,
        plays::update_play_item_name,
        // rules
        rules::get_rules,
        rules::update_rules,
        rules::get_rules_format,
        rules::get_rules_format_with_shuffle,
        rules::get_thumbnail_content_uid,
        // recents
        recents::update_content_by_used,
        recents::update_playlist_by_used,
        recents::get_recent_contents,
        recents::get_recent_playlists,
        // youtube
        youtube::import_youtube_video,
    ]
}
