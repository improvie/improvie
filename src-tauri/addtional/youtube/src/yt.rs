use std::{
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
};

use rusty_ytdl::{
    DownloadOptions, Video, VideoInfo, VideoOptions, VideoQuality, VideoSearchOptions,
    choose_format, search::Playlist,
};
use tokio::task::JoinHandle;

use crate::{YtPlaylistDownloadState, YtVideoDownloadState, model::SingleVideoDownload};

fn contents_dir(target_dir: &Path) -> std::io::Result<std::path::PathBuf> {
    let contents = target_dir.join("content");
    std::fs::create_dir_all(&contents)?;
    Ok(contents)
}

pub async fn download_single_video(
    video_url: &str,
    target_dir: &Path,
    callback: impl Fn(YtVideoDownloadState) -> Result<(), crate::YtError>,
) -> Result<SingleVideoDownload, crate::YtError> {
    let video = Video::new(video_url)?;
    let contents = contents_dir(target_dir)?;

    download_single_video_internal(video, contents, callback).await
}

pub async fn download_playlist(
    playlist_url: &str,
    target_dir: &Path,
    callback: impl Fn(YtPlaylistDownloadState) -> Result<(), crate::YtError> + Send + Sync + 'static,
) -> Result<Vec<SingleVideoDownload>, crate::YtError> {
    let playlist = Playlist::get(playlist_url, None).await?;
    let contents = contents_dir(target_dir)?;
    let callback = Arc::new(callback);

    let mut join_downloads = Vec::new();
    for (i, video) in playlist.videos.into_iter().enumerate() {
        let join: JoinHandle<Result<SingleVideoDownload, crate::YtError>> = tokio::spawn({
            let contents = contents.clone();
            let callback = Arc::clone(&callback);
            async move {
                let video = Video::new(&video.url)?;
                let download = download_single_video_internal(video, contents, |state| {
                    callback(YtPlaylistDownloadState { index: i, state })
                })
                .await?;
                Ok(download)
            }
        });

        join_downloads.push(join);
    }

    let mut downloads = Vec::new();
    for join in join_downloads {
        let download = join.await.map_err(|_| crate::YtError::JoinError)??;
        downloads.push(download);
    }

    Ok(downloads)
}

async fn download_single_video_internal(
    video: Video<'_>,
    contents: PathBuf,
    callback: impl Fn(YtVideoDownloadState) -> Result<(), crate::YtError>,
) -> Result<SingleVideoDownload, crate::YtError> {
    let info = video.get_info().await?;
    let title = info.video_details.title.clone();
    let id = info.video_details.video_id.clone();

    let video_path = contents.join(format!("{}.mp4", &id));
    let video_temp_path = contents.join(format!("{}.temp.mp4", &id));
    let audio_path = contents.join(format!("{}.mp3", &id));
    let thumbnail_path = contents.join(format!("{}.jpg", &id));

    let (video, audio, thumbnail) = tokio::join!(
        download_video(&video, &info, &video_temp_path, &callback),
        download_audio(&video, &info, &audio_path),
        download_thumbnail(&video, &info, &thumbnail_path),
    );
    video?;
    audio?;
    let has_thumbnail = thumbnail?;

    // TODO: add progress callback
    crate::ffmpeg::merge_video_audio(&video_temp_path, &audio_path, &video_path).await?;

    Ok(SingleVideoDownload {
        title,
        id,
        video_path,
        thumbnail_path: if has_thumbnail {
            Some(thumbnail_path)
        } else {
            None
        },
    })
}

async fn download_video(
    video: &Video<'_>,
    info: &VideoInfo,
    video_path: &Path,
    callback: impl Fn(YtVideoDownloadState) -> Result<(), crate::YtError>,
) -> Result<(), crate::YtError> {
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
        let state = YtVideoDownloadState {
            downloaded_mb: downloaded / 1024 / 1024,
            total_mb: total_size / 1024 / 1024,
            percentage: progress as u8,
        };
        callback(state)?;
    }

    Ok(())
}

async fn download_audio(
    video: &Video<'_>,
    info: &VideoInfo,
    audio_path: &Path,
) -> Result<(), crate::YtError> {
    let audio_format = choose_format(
        &info.formats,
        &VideoOptions {
            quality: VideoQuality::HighestAudio,
            filter: VideoSearchOptions::Audio,
            ..Default::default()
        },
    )?;

    audio_format
        .download(video.get_client(), &DownloadOptions::default(), &audio_path)
        .await?;

    Ok(())
}

/// Downloads the thumbnail of a video and saves it to the specified path.
///
/// # Return
///
/// Returns `true` if the thumbnail was downloaded successfully, `false` not found thumbnail.
async fn download_thumbnail(
    video: &Video<'_>,
    info: &VideoInfo,
    thumbnail_path: &Path,
) -> Result<bool, crate::YtError> {
    let thumbnail_url = info
        .video_details
        .thumbnails
        .iter()
        .max_by(|a, b| a.width.cmp(&b.width).then(a.height.cmp(&b.height)))
        .map(|t| t.url.clone());

    if let Some(thumbnail_url) = thumbnail_url {
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
    } else {
        Ok(false)
    }
}
