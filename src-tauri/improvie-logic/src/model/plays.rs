use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::logic::rule::Rule;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayItem {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayFolder {
    #[serde(flatten)]
    pub item: PlayItem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    #[serde(flatten)]
    pub item: PlayItem,

    pub thumbnail_path: Option<String>,
    pub rules: Vec<Rule>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct PlayFolderNode {
    pub folder: Uuid,
    pub children: Vec<PlayItemNode>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(tag = "kind")]
pub enum PlayItemNode {
    Folder { id: Uuid, sort_order: u32 },
    Playlist { id: Uuid, sort_order: u32 },
}
