use std::sync::Arc;

use improvie_app::command::{health_check, items, playlists};
use improvie_app::modules::Modules;
use improvie_app::state::AppState;
use improvie_infra::persistence::db::DbPool;
use tauri::{async_runtime::block_on, Manager};

mod init;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(init::log::init_log_plugin())
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {
            #[cfg(debug_assertions)]
            let data_dir = init::dev_folder();
            #[cfg(not(debug_assertions))]
            let data_dir = app.path().app_data_dir()?;

            let db = block_on(DbPool::new(data_dir))?;
            let modules = Modules::new(db);
            let modules = Arc::new(modules);
            let app_state = AppState { modules };
            app.manage(app_state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            health_check::health_check,
            items::get_items_hierarchy,
            items::get_contents,
            items::get_folders,
            items::create_folder,
            items::create_content,
            playlists::get_playlists,
            playlists::get_playlist_folders,
            playlists::get_favorite_playlists,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
