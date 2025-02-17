use tauri::{AppHandle, Runtime};
use tauri_plugin_dialog::DialogExt;

use crate::model::dialog::FileDialogResponse;

// TODO: check when paste an online url on windows

#[tauri::command]
pub async fn open_select_content_dialog<R: Runtime>(
    app: AppHandle<R>,
) -> Option<FileDialogResponse> {
    let path = app
        .dialog()
        .file()
        .set_title("Select Audio or Video")
        .add_filter("Audio or Video", &["mp3", "wav", "mp4"])
        .blocking_pick_file();

    path.map(FileDialogResponse::new)
}

#[tauri::command]
pub async fn open_select_thumbnail_dialog<R: Runtime>(
    app: AppHandle<R>,
) -> Option<FileDialogResponse> {
    let path = app
        .dialog()
        .file()
        .set_title("Select Image")
        .add_filter("Image", &["png", "jpeg", "gif"])
        .blocking_pick_file();

    path.map(FileDialogResponse::new)
}
