use std::time::Duration;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct Content {
    pub uid: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub seconds: Duration,
}
