use improvie_domain::model::items::{CreateBaseItemModel, CreateContentModel, CreateFolderModel};
use improvie_logic::{
    constant::items::ContentKind,
    model::items::{Content, FolderNode},
};
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

    pub kind: ContentKind,
    pub content_path: String,
    pub thumbnail_path: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateContentResponse {
    pub content: Content,
    pub folder_node: FolderNode,
}
