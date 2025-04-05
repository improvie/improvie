use tauri::{Runtime, plugin::TauriPlugin};
use tauri_plugin_log::{RotationStrategy, Target, TargetKind};

pub fn init_log_plugin<R: Runtime>() -> TauriPlugin<R> {
    tauri_plugin_log::Builder::new()
        .level(
            #[cfg(debug_assertions)]
            log::LevelFilter::Debug,
            #[cfg(not(debug_assertions))]
            log::LevelFilter::Info,
        )
        .targets(
            #[cfg(debug_assertions)]
            [
                Target::new(TargetKind::Stdout),
                Target::new(TargetKind::Folder {
                    path: super::dev_folder(),
                    file_name: Some(String::from("dev")),
                }),
                Target::new(TargetKind::Webview),
            ],
            #[cfg(not(debug_assertions))]
            [Target::new(TargetKind::LogDir { file_name: None })],
        )
        .rotation_strategy(RotationStrategy::KeepAll)
        .max_file_size(40000)
        .build()
}
