use improvie_app::model::items::{CreateBaseItemDto, CreateContentDto, CreateContentResponse};
use tauri::{AppHandle, Emitter};
use uid::Uid;
use youtube::YtError;

use crate::state::TauriAppState;

#[tauri::command]
pub async fn import_youtube_video<R: tauri::Runtime>(
    app: AppHandle<R>,
    state: TauriAppState<'_>,
    parent_folder_id: Uid,
    video_url: String,
) -> Result<CreateContentResponse, YtError> {
    let downloaded =
        youtube::download_single_video(&video_url, &state.document_dir, |downloading_state| {
            let _ = app.emit("yt-download-progress", downloading_state);
            Ok(())
        })
        .await?;

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
