use improvie_command::state::AppState;
use tauri::{Manager, async_runtime::block_on};

mod handler;
mod init;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default();

    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        builder = builder.plugin(tauri_plugin_window_state::Builder::new().build());
    }

    builder
        .plugin(tauri_plugin_dialog::init())
        .plugin(init::log::init_log_plugin())
        .setup(move |app| {
            let result = improvie_plugin::LOGGER.set((log::logger(), init::log::LOG_LEVEL_FILTER));

            if result.is_err() {
                log::error!("Failed to set logger. not logging on plugin");
            }

            #[cfg(all(debug_assertions, not(mobile)))]
            let data_dir = init::dev_folder();
            #[cfg(all(debug_assertions, not(mobile)))]
            let document_dir = data_dir.join("documents");

            #[cfg(not(all(debug_assertions, not(mobile))))]
            let data_dir = app.path().app_data_dir()?;
            #[cfg(not(all(debug_assertions, not(mobile))))]
            let document_dir = app.path().document_dir()?.join(&app.config().identifier);

            let app_state = block_on(AppState::new(data_dir, document_dir))?;
            app.manage(app_state);
            Ok(())
        })
        .invoke_handler(crate::handler::generate_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
