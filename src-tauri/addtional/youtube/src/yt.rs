use std::{path::PathBuf, sync::Arc};

use crate::{YtVideoRequest, YtVideoState, model::YtVideoDownloadComplete};

pub async fn download_single_video(
    client: reqwest::Client,
    request: YtVideoRequest,
    target_dir: PathBuf,
    callback: Arc<impl Fn(YtVideoState) -> bool + Send + Sync + 'static>,
) -> Result<bool, crate::YtError> {
    let final_video_path = target_dir.join(format!("{}.mp4", request.video_id));
    let download_video_path = target_dir.join(format!("{}.temp.mp4", request.video_id));
    let download_audio_path = target_dir.join(format!("{}.aac", request.video_id));
    let thumbnail_path = target_dir.join(format!("{}.jpg", request.video_id));

    let has_thumbnail = {
        let video_id = request.video_id.clone();
        let callback = Arc::clone(&callback);
        let video_url = request.video_url.clone();
        let audio_url = request.audio_url.clone();
        let thumbnail_url = request.thumbnail_url.clone();

        let thumbnail_path = thumbnail_path.clone();
        let download_video_path = download_video_path.clone();
        let download_audio_path = download_audio_path.clone();
        let video_process = tokio::spawn(crate::download::download_video(
            client.clone(),
            video_url,
            download_video_path,
            Arc::new(move |state| {
                callback(YtVideoState::Downloading {
                    video_id: video_id.clone(),
                    state,
                })
            }),
        ));
        let audio_process = tokio::spawn(crate::download::download_audio(
            client.clone(),
            audio_url,
            download_audio_path,
        ));
        let has_thumbnail = if let Some(thumbnail_url) = thumbnail_url {
            crate::download::download_thumbnail(client, thumbnail_url, thumbnail_path).await?;
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
            return Ok(false);
        }

        has_thumbnail
    };

    crate::ffmpeg::merge_video_audio(
        &download_video_path,
        &download_audio_path,
        &final_video_path,
    )
    .await?;

    std::fs::remove_file(&download_video_path)?;
    std::fs::remove_file(&download_audio_path)?;

    Ok(callback(YtVideoState::Completed {
        video_id: request.video_id.clone(),
        state: YtVideoDownloadComplete {
            title: request.content_title,
            video_path: final_video_path,
            thumbnail_path: if has_thumbnail {
                Some(thumbnail_path)
            } else {
                None
            },
        },
    }))
}
