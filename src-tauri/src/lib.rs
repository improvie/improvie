use improvie_command::modules::Modules;
use improvie_command::state::AppState;
use tauri::{Manager, async_runtime::block_on};

mod handler;
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

            let modules = block_on(Modules::new_with_db(data_dir))?;
            let app_state = AppState {
                modules,
                current_rules: Default::default(),
            };
            app.manage(app_state);
            Ok(())
        })
        .invoke_handler(crate::handler::generate_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
