use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::rule::Rule;

#[derive(Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub uid: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub plays: Vec<Play>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Play {
    pub uid: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub rules: Vec<Rule>,
}
