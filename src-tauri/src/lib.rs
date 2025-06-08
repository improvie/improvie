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

    builder = builder.plugin(tauri_plugin_dialog::init());
    builder = builder.plugin(init::log::init_log_plugin());

    builder
        .setup(move |app| {
            #[cfg(all(debug_assertions, not(mobile)))]
            let data_dir = init::dev_folder();

            #[cfg(not(all(debug_assertions, not(mobile))))]
            let data_dir = app.path().app_data_dir()?;

            let app_state = block_on(AppState::new(data_dir))?;

            app.manage(app_state);
            Ok(())
        })
        .invoke_handler(crate::handler::generate_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
