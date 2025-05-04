use improvie_logic::{AppResult, model::settings::AppSettings};

#[async_trait::async_trait]
pub trait SettingsRepository {
    async fn get_app_settings(&self) -> AppResult<AppSettings>;
    async fn set_app_settings(&self, settings: AppSettings) -> AppResult<()>;
}
