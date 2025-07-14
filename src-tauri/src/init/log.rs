use tauri::{Runtime, plugin::TauriPlugin};
use tauri_plugin_log::{RotationStrategy, Target, TargetKind};

#[cfg(debug_assertions)]
pub const LOG_LEVEL_FILTER: log::LevelFilter = log::LevelFilter::Debug;
#[cfg(not(debug_assertions))]
pub const LOG_LEVEL_FILTER: log::LevelFilter = log::LevelFilter::Info;

pub fn init_log_plugin<R: Runtime>() -> TauriPlugin<R> {
    tauri_plugin_log::Builder::new()
        .level(LOG_LEVEL_FILTER)
        .level_for("h2", log::LevelFilter::Info)
        .level_for("hyper", log::LevelFilter::Info)
        .level_for("hyper_util", log::LevelFilter::Info)
        .targets(
            #[cfg(all(debug_assertions, not(mobile)))]
            [
                Target::new(TargetKind::Stdout),
                Target::new(TargetKind::Folder {
                    path: super::dev_folder().join("logs"),
                    file_name: None,
                }),
                Target::new(TargetKind::Webview),
            ],
            #[cfg(not(all(debug_assertions, not(mobile))))]
            [
                Target::new(TargetKind::Stdout),
                Target::new(TargetKind::Webview),
                Target::new(TargetKind::LogDir { file_name: None }),
            ],
        )
        .rotation_strategy(RotationStrategy::KeepAll)
        .max_file_size(40000)
        .build()
}
