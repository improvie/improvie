use chrono::{DateTime, Utc};
use improvie_logic::constant::items::{ContentKind, ItemKind};
use improvie_logic::model::items::{Content, Folder, Item};
use more_convert::Convert;
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug)]
pub struct CurrentNodeRaw {
    pub child_id: Uuid,
    pub child_kind: ItemKind,
    pub sort_order: u32,
}

#[derive(sqlx::FromRow, Debug)]
pub struct NodeRaw {
    pub parent_folder_id: Uuid,
    pub child_id: Uuid,
    pub child_kind: ItemKind,
    pub sort_order: u32,
}

#[derive(sqlx::FromRow, Debug, Convert)]
#[convert(into(Item))]
pub struct ItemRaw {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Debug, Convert)]
#[convert(into(Content))]
pub struct ContentRaw {
    #[sqlx(flatten)]
    pub item: ItemRaw,

    pub kind: ContentKind,
    pub content_path: String,
    pub thumbnail_path: Option<String>,
}

#[derive(sqlx::FromRow, Debug, Convert)]
#[convert(into(Folder))]
pub struct FolderRaw {
    #[sqlx(flatten)]
    pub item: ItemRaw,
}
