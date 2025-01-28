use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Local};

use crate::{constant::items::ContentKind, Uuid};

#[derive(Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Local>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Content {
    #[serde(flatten)]
    pub item: Item,

    pub seconds: u64,
    pub kind: ContentKind,
    pub content_path: PathBuf,
    pub thumbnail_path: Option<PathBuf>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Folder {
    #[serde(flatten)]
    pub item: Item,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "detail")]
pub enum ItemDetailKind {
    Content(Content),
    Folder(Folder),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ItemDetail {
    #[serde(flatten)]
    pub item: Item,
    #[serde(flatten)]
    pub kind: ItemDetailKind,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct FolderNode {
    pub folder: Uuid,
    pub items: Vec<ItemNode>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum ItemNode {
    Folder { id: Uuid, sort_order: u32 },
    Content { id: Uuid, sort_order: u32 },
}
