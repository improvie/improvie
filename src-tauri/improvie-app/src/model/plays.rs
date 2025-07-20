use improvie_domain::model::plays::{
    CreateBasePlayItemModel, CreatePlayFolderModel, CreatePlaylistModel,
};
use improvie_logic::model::plays::{PlayFolder, PlayFolderNode, Playlist};
use more_convert::Convert;
use serde::{Deserialize, Serialize};
use uid::Uid;

#[derive(Debug, Deserialize, Convert)]
#[cfg_attr(feature = "ts", bind::dto("play"))]
#[convert(into(CreateBasePlayItemModel))]
pub struct CreateBasePlayItemDto {
    pub parent_folder_id: Uid,

    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Convert)]
#[cfg_attr(feature = "ts", bind::dto("play"))]
#[convert(into(CreatePlayFolderModel))]
pub struct CreatePlayFolderDto {
    #[serde(flatten)]
    pub item: CreateBasePlayItemDto,
}

#[derive(Debug, Deserialize, Convert)]
#[cfg_attr(feature = "ts", bind::dto("play"))]
#[convert(into(CreatePlaylistModel))]
pub struct CreatePlaylistDto {
    #[serde(flatten)]
    pub item: CreateBasePlayItemDto,

    pub thumbnail_path: Option<String>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "ts", bind::response("play"))]
pub struct CreatePlaylistResponse {
    pub playlist: Playlist,
    pub folder_node: PlayFolderNode,
}

#[derive(Debug, Serialize)]
#[cfg_attr(feature = "ts", bind::response("play"))]
pub struct CreatePlayFolderResponse {
    pub folder: PlayFolder,
    pub folder_node: PlayFolderNode,
}
