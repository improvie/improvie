use std::path::PathBuf;

use tauri::{AppHandle, Runtime};
use tauri_plugin_dialog::{DialogExt, FilePath};

use crate::model::dialog::NotAllowUrlOnFileDialog;

#[tauri::command]
pub async fn open_select_folder_dialog<R: Runtime>(
    app: AppHandle<R>,
) -> Result<Option<PathBuf>, NotAllowUrlOnFileDialog> {
    let path = app
        .dialog()
        .file()
        .set_title("Select Folder")
        .blocking_pick_folder();

    match path {
        Some(path) => match path {
            FilePath::Url(_) => Err(NotAllowUrlOnFileDialog),
            FilePath::Path(path) => Ok(Some(path)),
        },
        None => Ok(None),
    }
}
