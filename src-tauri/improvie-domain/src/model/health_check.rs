use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct SettingsModel {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
}
