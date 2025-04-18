use std::{io::Write, path::PathBuf, sync::Arc};

use ez_ffmpeg::{FfmpegContext, Output};
use improvie_app::model::items::{CreateBaseItemDto, CreateContentDto, CreateContentResponse};
use improvie_logic::impl_serialize_for_dyn_app_error;
use rusty_ytdl::{
    DownloadOptions, Video, VideoInfo, VideoOptions, VideoQuality, VideoSearchOptions,
    choose_format,
};
use tauri::{AppHandle, Emitter};
use uid::Uid;

use crate::state::TauriAppState;

#[derive(Debug, thiserror::Error, more_convert::VariantName)]
#[variant_name(prefix = "Yt")]
pub enum YtError {
    #[error("invalid url")]
    InvalidUrl,
    #[error("failed to download video")]
    Async,
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
    percentage: u8,
}

#[tauri::command]
pub async fn import_youtube_video<R: tauri::Runtime>(
    app: AppHandle<R>,
    state: TauriAppState<'_>,
    parent_folder_id: Uid,
    video_url: String,
) -> Result<CreateContentResponse, YtError> {
    let video = Video::new(video_url).map_err(|_| YtError::InvalidUrl)?;
    let video = Arc::new(video);

    let info = video.get_info().await?;
    let info = Arc::new(info);
    let title = info.video_details.title.clone();
    let id = info.video_details.video_id.clone();

    let contents = state.data_dir.join("content");

    std::fs::create_dir_all(&contents)?;

    let file_path = contents.join(format!("{}.mp4", &id));
    let tmp_path = contents.join(format!("{}.tmp.mp4", &id));

    let mut file_temp = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&tmp_path)?;

    let video_format = choose_format(
        &info.formats,
        &VideoOptions {
            quality: VideoQuality::HighestVideo,
            filter: VideoSearchOptions::Video,
            ..Default::default()
        },
    )?;
    let stream = video_format
        .stream(video.get_client(), &DownloadOptions::default())
        .await?;

    let total_size = stream.content_length() as u64;

    let mut downloaded: u64 = 0;

    let thumbnail = {
        let thumbnail_url = info
            .video_details
            .thumbnails
            .iter()
            .max_by(|a, b| a.width.cmp(&b.width).then(a.height.cmp(&b.height)))
            .map(|t| t.url.clone());

        let client = video.get_client().clone();
        let thumbnail_path = contents.join(format!("{}.jpg", &id));
        tokio::spawn(async move {
            if let Some(thumbnail_url) = thumbnail_url {
                let bytes = client
                    .get(thumbnail_url)
                    .send()
                    .await
                    .map_err(|_| YtError::Async)?
                    .bytes()
                    .await
                    .map_err(|_| YtError::Async)?;
                std::fs::write(&thumbnail_path, bytes)?;
                Result::<Option<PathBuf>, YtError>::Ok(Some(thumbnail_path))
            } else {
                Ok(None)
            }
        })
    };

    let audio_path = contents.join(format!("{}.mp3", &id));
    let audio_task = tokio::spawn(download_audio(
        audio_path.clone(),
        Arc::clone(&video),
        Arc::clone(&info),
    ));

    while let Some(chunk) = stream.chunk().await? {
        file_temp.write_all(&chunk)?;
        downloaded += chunk.len() as u64;
        let progress = (downloaded as f64 / total_size as f64) * 100.0;
        let _ = app.emit(
            "yt-download-progress",
            DownloadState {
                downloaded_mb: downloaded / 1024 / 1024,
                total_mb: total_size / 1024 / 1024,
                percentage: progress as u8,
            },
        );
    }

    file_temp.flush()?;

    audio_task.await.map_err(|_| YtError::Async)??;
    let thumbnail_path = thumbnail.await.map_err(|_| YtError::Async)??;

    FfmpegContext::builder()
        .input(tmp_path.to_string_lossy().to_string())
        .input(audio_path.to_string_lossy().to_string())
        .output(
            Output::new(file_path.to_string_lossy().to_string())
                .add_stream_map_with_copy("0:v?")
                .add_stream_map("1:a?")
                .set_audio_codec("aac")
                .set_format_opt("movflags", "faststart"),
        )
        .build()?
        .start()?
        .await?;

    std::fs::remove_file(&tmp_path)?;
    std::fs::remove_file(&audio_path)?;

    let dto = CreateContentDto {
        item: CreateBaseItemDto {
            parent_folder_id,
            title,
            description: None,
        },
        kind: improvie_logic::constant::items::ContentKind::Video,
        content_path: file_path.to_string_lossy().to_string(),
        thumbnail_path: thumbnail_path.map(|p| p.to_string_lossy().to_string()),
    };

    let result = state.modules.items_use_case().create_content(dto).await;
    match result {
        Ok(content) => Ok(content),
        Err(err) => Err(YtError::SaveError(err)),
    }
}

async fn download_audio(
    download_path: PathBuf,
    video: Arc<Video<'_>>,
    info: Arc<VideoInfo>,
) -> Result<(), YtError> {
    let audio_format = choose_format(
        &info.formats,
        &VideoOptions {
            quality: VideoQuality::HighestAudio,
            filter: VideoSearchOptions::Audio,
            ..Default::default()
        },
    )?;

    audio_format
        .download(
            video.get_client(),
            &DownloadOptions::default(),
            &download_path,
        )
        .await?;

    Ok(())
}
