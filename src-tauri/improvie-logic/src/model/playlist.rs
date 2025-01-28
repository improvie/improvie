use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{logic::rule::Rule, Uuid};

#[derive(Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub thumbnail_path: Option<PathBuf>,
    pub sort_order: u32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Play {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub rules: Vec<Rule>,
    pub sort_order: u32,
}
