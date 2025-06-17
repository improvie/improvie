use std::sync::Arc;

use tauri::{AppHandle, Emitter};
use youtube::{YtError, YtVideoRequest};

use crate::state::TauriAppState;

// TODO: add beautiful log

#[derive(Debug, thiserror::Error)]
#[error("YouTube error: {0}")]
pub struct YtErrorWrapper(#[from] pub YtError);

improvie_logic::impl_serialize_for_dyn_app_error!(
    YtErrorWrapper,
    kind = "YtError",
    message = "YouTube error"
);

#[tauri::command]
pub async fn import_youtube_video<R: tauri::Runtime>(
    app: AppHandle<R>,
    state: TauriAppState<'_>,
    request: YtVideoRequest,
) -> Result<(), YtErrorWrapper> {
    let downloaded = youtube::download_single_video(
        state.client.clone(),
        request,
        state.document_dir.clone(),
        Arc::new(move |downloading_state| {
            log::debug!("Video Downloading state: {:?}", downloading_state);
            let _ = app.emit("yt-downloading-state", downloading_state);
            true
        }),
    )
    .await?;

    log::info!("Video downloaded: {}", downloaded);

    Ok(())
}
