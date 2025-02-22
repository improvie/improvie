use serde::{Deserialize, Serialize};

use crate::{Uuid, logic::rule::Rule};

#[derive(Clone, Serialize, Deserialize)]
pub struct PlaylistFolder {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub sort_order: u32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub thumbnail_path: Option<String>,
    pub rules: Vec<Rule>,
    pub sort_order: u32,
}
