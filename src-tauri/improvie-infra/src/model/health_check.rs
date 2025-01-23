use chrono::{DateTime, Local};
use improvie_domain::model::health_check::SettingsModel;
use improvie_logic::Uuid;
use sqlx::prelude::FromRow;
use struct_convert::Convert;

#[derive(FromRow, Convert)]
#[convert(into = "SettingsModel")]
pub struct SettingsRow {
    pub id: Uuid,
    pub created_at: DateTime<Local>,
}
