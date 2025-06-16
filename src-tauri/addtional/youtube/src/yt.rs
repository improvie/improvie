use std::{io::Write, path::PathBuf, sync::Arc};

use crate::{YtVideoDownloading, YtVideoRequest, YtVideoState, model::YtVideoDownloadComplete};
use rusty_ytdl::{DownloadOptions, VideoOptions, VideoQuality, VideoSearchOptions, choose_format};

pub async fn download_single_video(
    request: YtVideoRequest,
    target_dir: PathBuf,
    callback: Arc<impl Fn(YtVideoState) -> bool + Send + Sync + 'static>,
) -> Result<bool, crate::YtError> {
    let final_video_path = target_dir.join(format!("{}.mp4", request.video_id));
    let download_video_path = target_dir.join(format!("{}.temp.mp4", request.video_id));
    let download_audio_path = target_dir.join(format!("{}.aac", request.video_id));
    let thumbnail_path = target_dir.join(format!("{}.jpg", request.video_id));

    enum Local {
        HasThumbnail(bool),
        CallbackFailed,
    }

    let local = tokio::task::LocalSet::new();
    let local: Local = local
        .run_until({
            let callback = Arc::clone(&callback);
            let video_url = request.video_url.clone();
            let audio_url = request.audio_url.clone();
            let thumbnail_url = request.thumbnail_url.clone();

            let thumbnail_path = thumbnail_path.clone();
            let download_video_path = download_video_path.clone();
            let download_audio_path = download_audio_path.clone();
            async move {
                let video_process = tokio::task::spawn_local(download_video(
                    video_url,
                    download_video_path,
                    Arc::new(move |state| callback(YtVideoState::Downloading(state))),
                ));
                let audio_process =
                    tokio::task::spawn_local(download_audio(audio_url, download_audio_path));
                let has_thumbnail = if let Some(thumbnail_url) = thumbnail_url {
                    download_thumbnail(thumbnail_url, thumbnail_path).await?;
                    true
                } else {
                    false
                };

                audio_process
                    .await
                    .map_err(|_| crate::YtError::JoinError)??;

                let is_callback_success = video_process
                    .await
                    .map_err(|_| crate::YtError::JoinError)??;

                if !is_callback_success {
                    return Ok(Local::CallbackFailed);
                }

                Ok::<Local, crate::YtError>(Local::HasThumbnail(has_thumbnail))
            }
        })
        .await?;

    let has_thumbnail = match local {
        Local::HasThumbnail(has_thumbnail) => has_thumbnail,
        Local::CallbackFailed => return Ok(false),
    };

    crate::ffmpeg::merge_video_audio(
        &download_video_path,
        &download_audio_path,
        &final_video_path,
    )
    .await?;

    std::fs::remove_file(&download_video_path)?;
    std::fs::remove_file(&download_audio_path)?;

    Ok(callback(YtVideoState::Completed(YtVideoDownloadComplete {
        title: request.content_title,
        video_path: final_video_path,
        thumbnail_path: if has_thumbnail {
            Some(thumbnail_path)
        } else {
            None
        },
    })))
}

async fn download_video(
    video_url: String,
    download_path: PathBuf,
    callback: Arc<impl Fn(YtVideoDownloading) -> bool + Sync + 'static>,
) -> Result<bool, crate::YtError> {
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

    let mut video_file = std::fs::File::create(video_path)?;

    while let Some(chunk) = stream.chunk().await? {
        video_file.write_all(&chunk)?;
        downloaded += chunk.len() as u64;
        let progress = (downloaded as f64 / total_size as f64) * 100.0;
        let state = YtVideoDownloading {
            downloaded_mb: downloaded / 1024 / 1024,
            total_mb: total_size / 1024 / 1024,
            percentage: progress as u8,
        };
        callback(state)?;
    }

    video_file.flush()?;
    drop(video_file);

    Ok(())
}

async fn download_audio(audio_url: String, audio_path: PathBuf) -> Result<(), crate::YtError> {
    let audio_format = choose_format(
        &info.formats,
        &VideoOptions {
            quality: VideoQuality::HighestAudio,
            filter: VideoSearchOptions::Audio,
            ..Default::default()
        },
    )?;

    audio_format
        .download(video.get_client(), &DownloadOptions::default(), audio_path)
        .await?;

    Ok(())
}

/// Downloads the thumbnail of a video and saves it to the specified path.
///
/// # Return
///
/// Returns `true` if the thumbnail was downloaded successfully, `false` not found thumbnail.
async fn download_thumbnail(
    thumbnail_url: String,
    thumbnail_path: PathBuf,
) -> Result<(), crate::YtError> {
    let client = video.get_client().clone();
    let bytes = client
        .get(thumbnail_url)
        .send()
        .await
        .map_err(|e| crate::YtError::Http(rusty_ytdl::VideoError::ReqwestMiddleware(e)))?
        .bytes()
        .await
        .map_err(|e| crate::YtError::Http(rusty_ytdl::VideoError::Reqwest(e)))?;
    std::fs::write(thumbnail_path, bytes)?;
    Ok(true)
}
