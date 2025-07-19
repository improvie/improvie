use chrono::{DateTime, Utc};
use improvie_logic::constant::items::{ContentKind, ItemKind};
use improvie_logic::model::items::{Content, Folder, Item};
use more_convert::Convert;
use uid::Uid;

#[derive(sea_orm::FromQueryResult, Debug)]
pub struct CurrentNodeRaw {
    pub child_id: Uid,
    pub child_kind: ItemKind,
    pub sort_order: u32,
}

#[derive(sea_orm::FromQueryResult, Debug)]
pub struct NodeRaw {
    pub parent_folder_id: Uid,
    pub child_id: Uid,
    pub child_kind: ItemKind,
    pub sort_order: u32,
}

#[derive(sea_orm::FromQueryResult, Debug, Convert)]
#[convert(into(Item))]
pub struct ItemRaw {
    pub id: Uid,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(sea_orm::FromQueryResult, Debug, Convert)]
#[convert(into(Content))]
pub struct ContentRaw {
    #[sea_orm(nested)]
    pub item: ItemRaw,

    pub kind: ContentKind,
    pub content_path: String,
    pub thumbnail_path: Option<String>,
}

#[derive(sea_orm::FromQueryResult, Debug, Convert)]
#[convert(into(Folder))]
pub struct FolderRaw {
    #[sea_orm(nested)]
    pub item: ItemRaw,
}
