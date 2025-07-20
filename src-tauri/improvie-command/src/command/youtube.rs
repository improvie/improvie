use std::sync::Arc;

use tauri::{AppHandle, Emitter};
use youtube::YtVideoRequest;

use crate::{model::yt::YtErrorWrapper, state::TauriAppState};

#[tauri::command]
pub async fn import_youtube_video<R: tauri::Runtime>(
    app: AppHandle<R>,
    state: TauriAppState<'_>,
    request: YtVideoRequest,
) -> Result<bool, YtErrorWrapper> {
    log::debug!(
        "Importing YouTube video with process id: {:?}",
        request.process_id
    );
    let downloaded = youtube::download_single_video(
        state.client.clone(),
        request,
        state.document_dir.clone(),
        Arc::new(move |downloading_state| {
            log::debug!("Video Downloading state: {downloading_state:?}");
            let _ = app.emit("yt-downloading-state", downloading_state);
            true
        }),
    )
    .await;

    match downloaded {
        Ok(true) => {
            log::info!("Video downloaded successfully");
            Ok(true)
        }
        Ok(false) => {
            log::warn!("Video download was not successful. failed on callback");
            Ok(false)
        }
        Err(e) => {
            log::error!("Error downloading video: {e:?}");
            Err(YtErrorWrapper(e))
        }
    }
}
