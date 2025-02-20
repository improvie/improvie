use std::path::PathBuf;

use improvie_logic::{impl_serialize_for_dyn_app_error, impl_unit_dyn_app_error};
use tauri_plugin_dialog::FilePath;

#[derive(Debug, thiserror::Error)]
#[error("Not allow url on file dialog")]
pub struct NotAllowUrlOnFileDialog;

impl_unit_dyn_app_error!(NotAllowUrlOnFileDialog);
impl_serialize_for_dyn_app_error!(NotAllowUrlOnFileDialog);

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum FileDialogKind {
    Audio,
    Video,
    Image,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct FileDialogResponse {
    pub path: PathBuf,
    pub name: String,
    pub kind: FileDialogKind,
}

impl FileDialogResponse {
    // filtered on file dialog, so unwrap and panic is unthinkable
    #[allow(clippy::unwrap_used)]
    pub fn new(path: FilePath) -> Result<Self, NotAllowUrlOnFileDialog> {
        match path {
            FilePath::Url(_) => Err(NotAllowUrlOnFileDialog),
            FilePath::Path(path_buf) => {
                let kind = match path_buf.extension().unwrap().to_string_lossy().as_ref() {
                    "mp3" | "wav" => FileDialogKind::Audio,
                    "mp4" => FileDialogKind::Video,
                    "png" | "jpeg" | "gif" => FileDialogKind::Image,
                    _ => unreachable!(),
                };

                let name = path_buf.file_stem().unwrap().to_string_lossy().to_string();

                Ok(Self {
                    path: path_buf,
                    name,
                    kind,
                })
            }
        }
    }
}
