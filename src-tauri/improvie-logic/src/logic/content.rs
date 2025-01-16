use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Local};
use uuid::Uuid;

use crate::constant::content::{ContentKind, ContentVis};

#[derive(Clone, Serialize, Deserialize)]
pub struct Content {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub seconds: u64,
    pub kind: ContentKind,
    pub vis: ContentVis,
    pub created_at: DateTime<Local>,
}
