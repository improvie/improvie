use more_convert::Convert;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Convert)]
#[convert(into(CreateBaseItemModel))]
pub struct CreateBaseItemDto {
    pub parent_folder_id: Uuid,

    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Convert)]
#[convert(into(CreateFolderModel))]
pub struct CreateFolderDto {
    #[serde(flatten)]
    pub item: CreateBaseItemDto,
}

#[derive(Debug, Deserialize, Convert)]
#[convert(into(CreateContentModel))]
pub struct CreateContentDto {
    #[serde(flatten)]
    pub item: CreateBaseItemDto,

    pub thumbnail_path: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateContentResponse {
    pub content: Content,
    pub folder_node: FolderNode,
}

#[derive(Debug, Serialize)]
pub struct CreateFolderResponse {
    pub folder: Folder,
    pub folder_node: FolderNode,
}
