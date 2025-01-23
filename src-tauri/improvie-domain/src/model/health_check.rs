use chrono::{DateTime, Local};
use improvie_logic::Uuid;

pub struct SettingsModel {
    pub id: Uuid,
    pub created_at: DateTime<Local>,
}
