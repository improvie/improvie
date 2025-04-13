use std::io::Write;

use futures_util::StreamExt;
use improvie_app::model::items::{CreateBaseItemDto, CreateContentDto, CreateContentResponse};
use improvie_logic::impl_serialize_for_dyn_app_error;
use tauri::{AppHandle, Emitter};
use uid::Uid;

use crate::state::TauriAppState;

#[derive(Debug, thiserror::Error, more_convert::VariantName)]
#[variant_name(prefix = "Yt")]
pub enum YtError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("content length not found")]
    ContentLength,
    #[error("errored on {0}")]
    Errored(String),
}

impl_serialize_for_dyn_app_error!(YtError);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct DownloadState {
    downloaded_mb: u64,
    total_mb: u64,
    percentage: f64,
}

#[tauri::command]
pub async fn import_youtube_video<R: tauri::Runtime>(
    app: AppHandle<R>,
    state: TauriAppState<'_>,
    parent_folder_id: Uid,
    title: String,
    video_url: String,
    // thumbnail_url: String,
) -> Result<CreateContentResponse, YtError> {
    let res = state.client.get(video_url).send().await?;
    let total_size = res.content_length().ok_or(YtError::ContentLength)?;

    let contents = state.data_dir.join("content");

    std::fs::create_dir_all(&contents)?;

    let file_path = contents.join(format!("{}.mp4", &title));
    let mut file = std::fs::File::create(&file_path)?;

    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;
        downloaded += chunk.len() as u64;
        let progress = (downloaded as f64 / total_size as f64) * 100.0;
        let _ = app.emit(
            "yt-download-progress",
            DownloadState {
                downloaded_mb: downloaded / 1024 / 1024,
                total_mb: total_size / 1024 / 1024,
                percentage: progress,
            },
        );
    }

    drop(file);

    let dto = CreateContentDto {
        item: CreateBaseItemDto {
            parent_folder_id,
            title,
            description: None,
        },
        kind: improvie_logic::constant::items::ContentKind::Video,
        content_path: file_path.to_string_lossy().to_string(),
        thumbnail_path: None,
        // thumbnail_path: Some(content.thumbnail_path.to_string_lossy().to_string()),
    };

    let result = state.modules.items_use_case().create_content(dto).await;
    match result {
        Ok(content) => Ok(content),
        Err(err) => Err(YtError::Errored(err.to_string())),
    }
}
