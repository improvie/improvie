use chrono::{DateTime, Utc};
use improvie_logic::{
    constant::plays::PlayItemKind,
    model::plays::{PlayFolder, PlayItem, Playlist},
};
use more_convert::Convert;
use uid::Uid;

#[derive(sea_orm::FromQueryResult, sea_orm::DerivePartialModel, Debug, Convert)]
#[convert(into(PlayItem))]
#[sea_orm(entity = "improvie_row::play_items::Entity")]
pub struct PlayItemRaw {
    pub id: Uid,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(sea_orm::FromQueryResult, Debug, Convert)]
#[convert(into(PlayFolder))]
pub struct PlayFolderRow {
    #[sea_orm(nested)]
    pub item: PlayItemRaw,
}

#[derive(sea_orm::FromQueryResult, Convert)]
#[convert(into(Playlist))]
pub struct PlaylistRow {
    #[sea_orm(nested)]
    pub item: PlayItemRaw,
    pub thumbnail_path: Option<String>,
}

#[derive(sea_orm::FromQueryResult)]
pub struct PlayCurrentNodeRaw {
    pub child_id: Uid,
    pub child_kind: PlayItemKind,
    pub sort_order: u32,
}

#[derive(sea_orm::FromQueryResult, Debug)]
pub struct PlayNodeRaw {
    pub parent_folder_id: Uid,
    pub child_id: Uid,
    pub child_kind: PlayItemKind,
    pub sort_order: u32,
}
