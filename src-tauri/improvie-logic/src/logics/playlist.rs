use uuid::Uuid;

use super::rule::Rule;

pub struct Playlist {
    pub uid: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub plays: Vec<Play>,
}

pub struct Play {
    pub uid: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub rules: Vec<Rule>,
}
