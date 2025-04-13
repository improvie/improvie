use improvie_command::state::AppState;
use tauri::{Manager, async_runtime::block_on};

mod handler;
mod init;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_cors_fetch::init())
        .plugin(init::log::init_log_plugin())
        .setup(move |app| {
            let result = improvie_plugin::LOGGER.set((log::logger(), init::log::LOG_LEVEL_FILTER));

            if result.is_err() {
                log::error!("Failed to set logger. not logging on plugin");
            }

            #[cfg(debug_assertions)]
            let data_dir = init::dev_folder();
            #[cfg(not(debug_assertions))]
            let data_dir = app.path().app_data_dir()?;

            let app_state = block_on(AppState::new(data_dir))?;
            app.manage(app_state);
            Ok(())
        })
        .invoke_handler(crate::handler::generate_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
