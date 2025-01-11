use improvie_app::modules::UsecasesModule;
use tauri::{async_runtime::block_on, Manager};
use tauri_plugin_log::{Target, TargetKind, TimezoneStrategy};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

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
            let usecases = block_on(UsecasesModule::new(data_dir))?;
            block_on(usecases.health_check_usecase().health_check())?;
            app.manage(usecases);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
