use improvie_app::model::items::{CreateBaseItemDto, CreateContentDto, CreateContentResponse};
use tauri::{AppHandle, Emitter};
use uid::Uid;
use youtube::{SingleVideoDownload, YtError};

use crate::state::TauriAppState;

async fn save(
    state: &TauriAppState<'_>,
    parent_folder_id: Uid,
    downloaded: SingleVideoDownload,
) -> Result<CreateContentResponse, YtError> {
    let dto = CreateContentDto {
        item: CreateBaseItemDto {
            parent_folder_id,
            title: downloaded.title,
            description: None,
        },
        kind: improvie_logic::constant::items::ContentKind::Video,
        content_path: downloaded.video_path.to_string_lossy().to_string(),
        thumbnail_path: downloaded
            .thumbnail_path
            .map(|p| p.to_string_lossy().to_string()),
    };

    let result = state.modules.items_use_case().create_content(dto).await;
    match result {
        Ok(content) => Ok(content),
        Err(err) => Err(YtError::SaveError(err)),
    }
}

// TODO: add beautiful log

#[tauri::command]
pub async fn import_youtube_video<R: tauri::Runtime>(
    app: AppHandle<R>,
    state: TauriAppState<'_>,
    parent_folder_id: Uid,
    video_url_or_id: String,
) -> Result<CreateContentResponse, YtError> {
    let downloaded = youtube::download_single_video(
        &video_url_or_id,
        state.document_dir.to_path_buf(),
        |downloading_state| {
            log::debug!("Video Downloading state: {:?}", downloading_state);
            let _ = app.emit("yt-download-progress-video", downloading_state);
            Ok(())
        },
    )
    .await?;

    save(&state, parent_folder_id, downloaded).await
}

#[tauri::command]
pub async fn import_youtube_playlist<R: tauri::Runtime>(
    app: AppHandle<R>,
    state: TauriAppState<'_>,
    parent_folder_id: Uid,
    playlist_url: String,
) -> Result<Vec<CreateContentResponse>, YtError> {
    let downloaded = youtube::download_playlist(
        &playlist_url,
        state.document_dir.to_path_buf(),
        move |downloading_state| {
            log::debug!("Playlist Downloading state: {:?}", downloading_state);
            let _ = app.emit("yt-download-progress-playlist", downloading_state);
            Ok(())
        },
    )
    .await?;

    let mut contents = Vec::new();
    for downloaded in downloaded {
        let content = save(&state, parent_folder_id, downloaded).await?;
        contents.push(content);
    }

    Ok(contents)
}
