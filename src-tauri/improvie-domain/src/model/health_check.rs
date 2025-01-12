use chrono::{DateTime, Local};
use uuid::Uuid;

pub struct SettingsModel {
    pub id: Uuid,
    pub created_at: DateTime<Local>,
}
