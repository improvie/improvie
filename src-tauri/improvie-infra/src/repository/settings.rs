use improvie_domain::repository::settings::SettingsRepository;
use improvie_logic::model::settings::AppSettings;
use uid::Uid;

use crate::model::settings::AppSettingsRow;

use super::def_repository_impl;

def_repository_impl!(SettingsRepositoryImpl);

#[async_trait::async_trait]
impl SettingsRepository for SettingsRepositoryImpl {
    async fn get_app_settings(&self) -> improvie_logic::DynAppResult<AppSettings> {
        let row: AppSettingsRow = sqlx::query_as::<_, AppSettingsRow>(
            "SELECT id, settings, created_at FROM app_settings WHERE id = ?",
        )
        .bind(Uid::nil())
        .fetch_one(&self.db.pool())
        .await?;

        Ok(row.settings)
    }

    async fn set_app_settings(&self, settings: AppSettings) -> improvie_logic::DynAppResult<()> {
        sqlx::query("UPDATE app_settings SET settings = ? WHERE id = ?")
            .bind(sqlx::types::Json(settings))
            .bind(Uid::nil())
            .execute(&self.db.pool())
            .await?;

        Ok(())
    }
}
