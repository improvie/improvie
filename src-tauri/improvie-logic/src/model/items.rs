use crate::constant::items::ContentKind;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uid::Uid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: Uid,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    #[serde(flatten)]
    pub item: Item,

    pub kind: ContentKind,
    pub content_path: String,
    pub thumbnail_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
    #[serde(flatten)]
    pub item: Item,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct FolderNode {
    pub folder: Uid,
    pub items: Vec<ItemNode>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(tag = "kind")]
pub enum ItemNode {
    Folder { id: Uid, sort_order: u32 },
    Content { id: Uid, sort_order: u32 },
}
