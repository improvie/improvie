use improvie_domain::{
    modules::RepositoriesModule, persistence::db::DbTx, repository::settings::SettingsRepository,
};
use improvie_logic::{DynAppResult, model::settings::AppSettings};

super::def_use_case!(SettingsUseCase);

impl<R: RepositoriesModule> SettingsUseCase<R> {
    pub async fn health_check(&self) -> DynAppResult<()> {
        self.repository
            .settings_repository()
            .get_app_settings(self.repository.connection())
            .await
            .map(|_| ())
    }

    pub async fn get_app_settings(&self) -> DynAppResult<AppSettings> {
        let option = self
            .repository
            .settings_repository()
            .get_app_settings(self.repository.connection())
            .await?;

        option.ok_or_else(|| {
            self.repository
                .record_not_found(String::from("App settings not found"))
        })
    }

    pub async fn set_app_settings(&self, settings: AppSettings) -> DynAppResult<()> {
        let tx = self.repository.begin().await?;
        let conn = tx.connection();
        let result = self
            .repository
            .settings_repository()
            .set_app_settings(conn, settings)
            .await;

        super::tx_commit!(tx, result)
    }
}
