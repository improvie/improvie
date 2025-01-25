use chrono::{DateTime, Local};
use improvie_domain::model::health_check::SettingsModel;
use improvie_logic::Uuid;
use more_convert::Convert;
use sqlx::prelude::FromRow;

#[derive(FromRow, Convert)]
#[convert(into(SettingsModel))]
pub struct SettingsRow {
    pub id: Uuid,
    pub created_at: DateTime<Local>,
}
