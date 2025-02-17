use tauri_plugin_dialog::FilePath;

#[derive(Debug, Clone, serde::Serialize)]
pub struct FileDialogResponse {
    pub path: FilePath,
    pub name: String,
}

impl FileDialogResponse {
    pub fn new(path: FilePath) -> Self {
        let name = match &path {
            FilePath::Url(_) => {
                // TODO: Maybe we'll get the title at reqwest.
                String::from("not support auto name from url")
            }
            FilePath::Path(path_buf) => path_buf
                .file_stem()
                .unwrap_or_else(|| path_buf.as_os_str())
                .to_string_lossy()
                .to_string(),
        };
        Self { path, name }
    }
}
