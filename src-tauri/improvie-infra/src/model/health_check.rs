use chrono::{DateTime, Utc};
use improvie_domain::model::health_check::SettingsModel;
use more_convert::Convert;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(FromRow, Convert)]
#[convert(into(SettingsModel))]
pub struct SettingsRow {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
}
