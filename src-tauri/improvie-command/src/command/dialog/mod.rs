use std::path::PathBuf;

use tauri_plugin_dialog::FilePath;

use crate::model::dialog::NotAllowUrlOnFileDialog;

pub mod file;

fn into_path(file_path: FilePath) -> Result<PathBuf, NotAllowUrlOnFileDialog> {
    match file_path {
        FilePath::Url(url) => url.to_file_path().map_err(|_| NotAllowUrlOnFileDialog),
        FilePath::Path(path_buf) => Ok(path_buf),
    }
}
