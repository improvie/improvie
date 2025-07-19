use improvie_domain::{modules::RepositoriesModule, repository::settings::SettingsRepository};
use improvie_logic::{DynAppResult, model::settings::AppSettings};

super::def_use_case!(SettingsUseCase);

impl<R: RepositoriesModule> SettingsUseCase<R> {
    pub async fn health_check(&self) -> DynAppResult<()> {
        self.repository
            .settings_repository()
            .get_app_settings()
            .await
            .map(|_| ())
    }

    pub async fn get_app_settings(&self) -> DynAppResult<AppSettings> {
        let option = self
            .repository
            .settings_repository()
            .get_app_settings()
            .await?;

        option.ok_or_else(|| self.repository.record_not_found())
    }

    pub async fn set_app_settings(&self, settings: AppSettings) -> DynAppResult<()> {
        self.repository
            .settings_repository()
            .set_app_settings(settings)
            .await
    }
}
