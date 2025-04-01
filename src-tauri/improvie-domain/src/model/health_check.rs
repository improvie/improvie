use chrono::{DateTime, Utc};
use uid::Uid;

pub struct SettingsModel {
    pub id: Uid,
    pub created_at: DateTime<Utc>,
}
