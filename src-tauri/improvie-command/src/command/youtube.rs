use std::io::Write;

use ez_ffmpeg::{FfmpegContext, Output};
use improvie_app::model::items::{CreateBaseItemDto, CreateContentDto, CreateContentResponse};
use improvie_logic::impl_serialize_for_dyn_app_error;
use rusty_ytdl::{Video, VideoOptions, VideoQuality, VideoSearchOptions};
use tauri::{AppHandle, Emitter};
use uid::Uid;

use crate::state::TauriAppState;

#[derive(Debug, thiserror::Error, more_convert::VariantName)]
#[variant_name(prefix = "Yt")]
pub enum YtError {
    #[error("invalid url")]
    InvalidUrl,
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("http error: {0}")]
    Http(#[from] rusty_ytdl::VideoError),
    #[error("failed to create content: {0}")]
    SaveError(#[from] improvie_logic::AppError),
    #[error("failed to create ffmpeg context: {0}")]
    Ffmpeg(#[from] ez_ffmpeg::error::Error),
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
    video_url: String,
) -> Result<CreateContentResponse, YtError> {
    let video = Video::new_with_options(
        video_url,
        VideoOptions {
            quality: VideoQuality::HighestVideo,
            filter: VideoSearchOptions::Video,
            ..Default::default()
        },
    )
    .map_err(|_| YtError::InvalidUrl)?;

    let info = video.get_info().await?;
    let title = info.video_details.title.clone();
    let id = info.video_details.video_id.clone();

    let contents = state.data_dir.join("content");

    std::fs::create_dir_all(&contents)?;

    let file_path = contents.join(format!("{}.mp4", &id));
    let tmp_path = contents.join(format!("{}.mp4.tmp", &id));

    log::debug!("create tmp file: {:?}", tmp_path);

    let mut file_temp = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&tmp_path)?;

    let stream = video.stream().await?;

    let total_size = stream.content_length() as u64;

    let mut downloaded: u64 = 0;

    while let Some(chunk) = stream.chunk().await? {
        file_temp.write_all(&chunk)?;
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

    file_temp.flush()?;

    FfmpegContext::builder()
        .input(tmp_path.display().to_string())
        .output(
            Output::new(file_path.display().to_string())
                .add_stream_map_with_copy("0:v?")
                .add_stream_map_with_copy("0:a?")
                .set_format_opt("movflags", "faststart"),
        )
        .build()?
        .start()?
        .await?;

    std::fs::remove_file(&tmp_path)?;

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
        Err(err) => Err(YtError::SaveError(err)),
    }
}
