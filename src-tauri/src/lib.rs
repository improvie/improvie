use command::{
    health_check::health_check,
    items::{create_content, create_folder, get_contents, get_folders, get_items_hierarchy},
    playlists::{get_favorite_playlists, get_playlists, get_playlist_folders},
};
use improvie_infra::persistence::db::DbPool;
use modules::Modules;
use tauri::{async_runtime::block_on, Manager};
use tauri_plugin_log::{Target, TargetKind, TimezoneStrategy};

mod command;
mod modules;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(feature = "dev")]
    #[allow(clippy::unwrap_used)]
    let dev_data_dir = std::path::Path::new(std::env!("CARGO_MANIFEST_DIR")).join("dev");

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(
                    #[cfg(feature = "dev")]
                    log::LevelFilter::Debug,
                    #[cfg(not(feature = "dev"))]
                    log::LevelFilter::Info,
                )
                .timezone_strategy(TimezoneStrategy::UseLocal)
                .targets(
                    #[cfg(feature = "dev")]
                    [
                        Target::new(TargetKind::Stdout),
                        Target::new(TargetKind::Folder {
                            path: dev_data_dir.clone(),
                            file_name: Some(String::from("dev")),
                        }),
                        Target::new(TargetKind::Webview),
                    ],
                    #[cfg(not(feature = "dev"))]
                    [Target::new(TargetKind::LogDir { file_name: None })],
                )
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {
            #[cfg(feature = "dev")]
            let data_dir = dev_data_dir.clone();
            #[cfg(not(feature = "dev"))]
            let data_dir = app.path().app_data_dir()?;
            let db = block_on(DbPool::new(data_dir))?;
            let repositories = Modules::new(db);
            app.manage(repositories);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            health_check,
            get_items_hierarchy,
            get_contents,
            get_folders,
            get_playlists,
            get_playlist_folders,
            get_favorite_playlists,
            create_folder,
            create_content,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
