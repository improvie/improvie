use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Local};
use uuid::Uuid;

use crate::constant::content::ContentKind;

#[derive(Clone, Serialize, Deserialize)]
pub struct Content {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub seconds: u64,
    pub kind: ContentKind,
    pub path: PathBuf,
    pub created_at: DateTime<Local>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Folder {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Local>,
}
