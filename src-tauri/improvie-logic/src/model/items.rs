use crate::constant::items::ContentKind;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uid::Uid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", ts_bind::ts("item/index.ts"))]
pub struct Item {
    pub id: Uid,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", ts_bind::ts("item/index.ts"))]
pub struct Content {
    #[serde(flatten)]
    pub item: Item,

    pub kind: ContentKind,
    pub content_path: String,
    pub thumbnail_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", ts_bind::ts("item/index.ts"))]
pub struct Folder {
    #[serde(flatten)]
    pub item: Item,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "ts", ts_bind::ts("item/index.ts"))]
pub struct FolderNode {
    pub folder: Uid,
    pub items: Vec<ItemNode>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "ts", ts_bind::ts("item/index.ts"))]
#[serde(tag = "kind")]
pub enum ItemNode {
    Folder { id: Uid, sort_order: u32 },
    Content { id: Uid, sort_order: u32 },
}
