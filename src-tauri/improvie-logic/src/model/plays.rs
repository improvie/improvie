use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use uid::Uid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayItem {
    pub id: Uid,
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
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct PlayFolderNode {
    pub folder: Uid,
    pub children: Vec<PlayItemNode>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(tag = "kind")]
pub enum PlayItemNode {
    Folder { id: Uid, sort_order: u32 },
    Playlist { id: Uid, sort_order: u32 },
}
