use improvie_domain::{modules::RepositoriesModule, repository::settings::SettingsRepository};
use improvie_logic::{AppResult, model::settings::AppSettings};

super::def_use_case!(SettingsUseCase);

impl<R: RepositoriesModule> SettingsUseCase<R> {
    pub async fn health_check(&self) -> AppResult<()> {
        self.repository
            .settings_repository()
            .get_app_settings()
            .await
            .map(|_| ())
    }

    pub async fn get_app_settings(&self) -> AppResult<AppSettings> {
        self.repository
            .settings_repository()
            .get_app_settings()
            .await
    }

    pub async fn set_app_settings(&self, settings: AppSettings) -> AppResult<()> {
        self.repository
            .settings_repository()
            .set_app_settings(settings)
            .await
    }
}
