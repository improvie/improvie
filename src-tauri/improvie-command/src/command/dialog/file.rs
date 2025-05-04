use std::path::PathBuf;

use tauri::{AppHandle, Runtime};
use tauri_plugin_dialog::{DialogExt, FilePath};

use crate::model::dialog::{
    AUDIO_FILTER, ContentFileDialogResponse, IMAGE_FILTER, NotAllowUrlOnFileDialog, VIDEO_FILTER,
};

// TODO: check when paste an online url on windows

#[tauri::command]
pub async fn open_select_content_dialog<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Option<ContentFileDialogResponse>, NotAllowUrlOnFileDialog> {
    let path = app
        .dialog()
        .file()
        .set_title("Select Audio or Video")
        .add_filter("Audio or Video", &[AUDIO_FILTER, VIDEO_FILTER].concat())
        .blocking_pick_file();

    path.map(ContentFileDialogResponse::new).transpose()
}

#[tauri::command]
pub async fn open_select_thumbnail_dialog<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Option<PathBuf>, NotAllowUrlOnFileDialog> {
    let path = app
        .dialog()
        .file()
        .set_title("Select Image")
        .add_filter("Image", IMAGE_FILTER)
        .blocking_pick_file();

    match path {
        Some(path) => match path {
            FilePath::Url(_) => Err(NotAllowUrlOnFileDialog),
            FilePath::Path(path) => Ok(Some(path)),
        },
        None => Ok(None),
    }
}
