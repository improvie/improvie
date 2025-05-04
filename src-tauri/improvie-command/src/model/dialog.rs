use std::path::PathBuf;

use improvie_logic::{constant::items::ContentKind, def_unit_dyn_app_error};

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
    /// Create a new [`ContentFileDialogResponse`] from a [`PathBuf`].
    /// Importantny, this function will check the file extension to determine the content kind.
    pub fn new(path: PathBuf) -> Self {
        let Some(ext) = path.extension() else {
            unreachable!("File dialog should return a file with extension")
        };
        let ext = ext.to_string_lossy();
        let ext = ext.as_ref();

        let kind = if AUDIO_FILTER.contains(&ext) {
            ContentKind::Audio
        } else if VIDEO_FILTER.contains(&ext) {
            ContentKind::Video
        } else {
            unreachable!("File dialog should return a file with audio or video extension")
        };

        #[allow(clippy::unwrap_used)]
        let name = path.file_stem().unwrap().to_string_lossy().to_string();

        Self { path, name, kind }
    }
}
