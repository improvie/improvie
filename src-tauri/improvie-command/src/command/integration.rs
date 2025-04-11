use improvie_app::model::items::{CreateBaseItemDto, CreateContentDto, CreateContentResponse};
use improvie_logic::impl_serialize_for_dyn_app_error;
use improvie_yt::YtStore;
use tauri::{AppHandle, Emitter};
use uid::Uid;

use crate::state::TauriAppState;

#[derive(Debug, thiserror::Error, more_convert::VariantName)]
#[variant_name(prefix = "Yt")]
pub enum YtError {
    #[error("youtube integration is loading")]
    Loading,
    #[error("youtube integration errored: {0}")]
    Errored(String),
}

impl_serialize_for_dyn_app_error!(YtError);

#[tauri::command]
pub async fn import_youtube_video<R: tauri::Runtime>(
    app: AppHandle<R>,
    state: TauriAppState<'_>,
    parent_folder_id: Uid,
    url: String,
) -> Result<CreateContentResponse, YtError> {
    let yt = match &*state.yt.read().await {
        YtStore::Loaded(yt) => yt.clone(),
        YtStore::Loading => {
            return Err(YtError::Loading);
        }
        YtStore::Error(err) => {
            return Err(YtError::Errored(err.to_string()));
        }
    };

    let content = match yt
        .download_content_with_progress(url, move |progress| {
            let _ = app.emit("yt-download-progress", progress);
        })
        .await
    {
        Ok(content) => content,
        Err(err) => return Err(YtError::Errored(err.to_string())),
    };

    let dto = CreateContentDto {
        item: CreateBaseItemDto {
            parent_folder_id,
            title: content.title,
            description: None,
        },
        kind: improvie_logic::constant::items::ContentKind::Video,
        content_path: content.content_path.to_string_lossy().to_string(),
        thumbnail_path: Some(content.thumbnail_path.to_string_lossy().to_string()),
    };

    let result = state.modules.items_use_case().create_content(dto).await;
    match result {
        Ok(content) => Ok(content),
        Err(err) => Err(YtError::Errored(err.to_string())),
    }
}
