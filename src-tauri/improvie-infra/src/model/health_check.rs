use chrono::{DateTime, Utc};
use improvie_logic::model::settings::AppSettings;
use more_convert::Convert;
use sqlx::prelude::FromRow;
use uid::Uid;

#[derive(FromRow, Convert)]
pub struct AppSettingsRow {
    pub id: Uid,
    #[sqlx(json)]
    pub settings: AppSettings,
    pub created_at: DateTime<Utc>,
}
