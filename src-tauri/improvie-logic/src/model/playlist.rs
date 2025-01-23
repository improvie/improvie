use serde::{Deserialize, Serialize};

use crate::{logic::rule::Rule, Uuid};

#[derive(Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub emoji: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Play {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub rules: Vec<Rule>,
}
