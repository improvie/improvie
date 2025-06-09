use improvie_logic::{DynAppResult, model::settings::AppSettings};

#[async_trait::async_trait]
pub trait SettingsRepository {
    async fn get_app_settings(&self) -> DynAppResult<AppSettings>;
    async fn set_app_settings(&self, settings: AppSettings) -> DynAppResult<()>;
}
