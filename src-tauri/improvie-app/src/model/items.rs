use improvie_domain::model::items::{CreateBaseItemModel, CreateContentModel, CreateFolderModel};
use improvie_logic::{
    constant::items::ContentKind,
    model::items::{Content, Folder, FolderNode},
};
use more_convert::Convert;
use serde::{Deserialize, Serialize};
use uid::Uid;

#[derive(Debug, Deserialize, Convert)]
#[cfg_attr(feature = "ts", bind::dto("item"))]
#[convert(into(CreateBaseItemModel))]
pub struct CreateBaseItemDto {
    pub parent_folder_id: Uid,

    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Convert)]
#[cfg_attr(feature = "ts", bind::dto("item"))]
#[convert(into(CreateFolderModel))]
pub struct CreateFolderDto {
    #[serde(flatten)]
    pub item: CreateBaseItemDto,
}

#[derive(Debug, Deserialize, Convert)]
#[cfg_attr(feature = "ts", bind::dto("item"))]
#[convert(into(CreateContentModel))]
pub struct CreateContentDto {
    #[serde(flatten)]
    pub item: CreateBaseItemDto,

    pub kind: ContentKind,
    pub content_path: String,
    pub thumbnail_path: Option<String>,
    pub seconds: u32,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "ts", bind::response("item"))]
pub struct CreateContentResponse {
    pub content: Content,
    pub folder_node: FolderNode,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "ts", bind::response("item"))]
pub struct CreateFolderResponse {
    pub folder: Folder,
    pub folder_node: FolderNode,
}
