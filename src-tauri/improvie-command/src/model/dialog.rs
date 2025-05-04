use std::path::PathBuf;

use improvie_logic::{constant::items::ContentKind, def_unit_dyn_app_error};
use tauri_plugin_dialog::FilePath;

def_unit_dyn_app_error! {
    pub struct NotAllowUrlOnFileDialog = "Not allow url on file dialog";
}

pub(crate) const IMAGE_FILTER: &[&str] = &["png", "jpeg", "gif"];
pub(crate) const AUDIO_FILTER: &[&str] = &["mp3", "wav", "aac"];
pub(crate) const VIDEO_FILTER: &[&str] = &["mp4"];

#[derive(Debug, Clone, serde::Serialize)]
#[cfg_attr(feature = "ts", bind::ts("file-dialog.ts"))]
pub struct ContentFileDialogResponse {
    pub path: PathBuf,
    pub name: String,
    pub kind: ContentKind,
}

impl ContentFileDialogResponse {
    pub fn new(path: FilePath) -> Result<Self, NotAllowUrlOnFileDialog> {
        match path {
            FilePath::Url(_) => Err(NotAllowUrlOnFileDialog),
            FilePath::Path(path_buf) => {
                let Some(ext) = path_buf.extension() else {
                    return Err(NotAllowUrlOnFileDialog);
                };
                let ext = ext.to_string_lossy();
                let ext = ext.as_ref();

                let kind = if AUDIO_FILTER.contains(&ext) {
                    ContentKind::Audio
                } else if VIDEO_FILTER.contains(&ext) {
                    ContentKind::Video
                } else {
                    return Err(NotAllowUrlOnFileDialog);
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
