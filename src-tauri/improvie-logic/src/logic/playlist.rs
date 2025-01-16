use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::constant::Visibility;

use super::rule::Rule;

#[derive(Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub vis: Visibility,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Play {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub rules: Vec<Rule>,
}
