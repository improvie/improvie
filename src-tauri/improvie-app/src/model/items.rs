use improvie_domain::model::items::{CreateBaseItemModel, CreateContentModel, CreateFolderModel};
use improvie_logic::{constant::items::ContentKind, Uuid};
use more_convert::Convert;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Convert)]
#[convert(into(CreateBaseItemModel))]
pub struct CreateBaseItemDto {
    pub parent_folder_id: Uuid,
    pub sort_order: u32,

    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Convert)]
#[convert(into(CreateFolderModel))]
pub struct CreateFolderDto {
    #[serde(flatten)]
    pub item: CreateBaseItemDto,
}

#[derive(Debug, Serialize, Deserialize, Convert)]
#[convert(into(CreateContentModel))]
pub struct CreateContentDto {
    #[serde(flatten)]
    pub item: CreateBaseItemDto,

    pub kind: ContentKind,
    pub content_path: String,
    pub thumbnail_path: Option<String>,
}
