use std::path::PathBuf;

use improvie_logic::def_unit_dyn_app_error;
use tauri_plugin_dialog::FilePath;

def_unit_dyn_app_error! {
    pub struct NotAllowUrlOnFileDialog = "Not allow url on file dialog";
}

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
    pub fn new(path: FilePath) -> Result<Self, NotAllowUrlOnFileDialog> {
        match path {
            FilePath::Url(_) => Err(NotAllowUrlOnFileDialog),
            FilePath::Path(path_buf) => {
                let Some(ext) = path_buf.extension() else {
                    return Err(NotAllowUrlOnFileDialog);
                };
                let kind = match ext.to_string_lossy().as_ref() {
                    "mp3" | "wav" => FileDialogKind::Audio,
                    "mp4" => FileDialogKind::Video,
                    "png" | "jpeg" | "gif" => FileDialogKind::Image,
                    _ => unreachable!(),
                };

                #[allow(clippy::unwrap_used)]
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
