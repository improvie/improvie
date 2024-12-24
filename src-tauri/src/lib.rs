use tauri_plugin_log::{Target, TargetKind, TimezoneStrategy};

mod state;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(if cfg!(feature = "dev") {
                    log::LevelFilter::Debug
                } else {
                    log::LevelFilter::Info
                })
                .timezone_strategy(TimezoneStrategy::UseLocal)
                .targets(
                    #[cfg(not(feature = "dev"))]
                    [
                        Target::new(TargetKind::Stdout),
                        Target::new(TargetKind::Folder {
                            #[allow(clippy::unwrap_used)]
                            path: std::env::current_dir().unwrap(),
                            file_name: Some(String::from("dev")),
                        }),
                        Target::new(TargetKind::Webview),
                    ],
                    #[cfg(feature = "dev")]
                    [Target::new(TargetKind::LogDir { file_name: None })],
                )
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
