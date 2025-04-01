use improvie_app::model::items::{CreateBaseItemDto, CreateContentDto, CreateContentResponse};
use improvie_logic::{AppError, AppResult};
use improvie_yt::YtStore;
use uid::Uid;

use crate::state::TauriAppState;

#[tauri::command]
pub async fn import_youtube_video(
    state: TauriAppState<'_>,
    parent_folder_id: Uid,
    url: String,
) -> AppResult<CreateContentResponse> {
    let yt = match &*state.yt.read().await {
        YtStore::Loaded(yt) => yt.clone(),
        YtStore::Loading => {
            return Err(AppError::NotReady(
                "youtube integration",
                "now loading".to_string(),
            ));
        }
        YtStore::Error(err) => {
            return Err(AppError::Errored("youtube integration", err.to_string()));
        }
    };

    let content = match yt.download_content(url).await {
        Ok(content) => content,
        Err(err) => return Err(AppError::Errored("youtube integration", err.to_string())),
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

    state.modules.items_use_case().create_content(dto).await
}
