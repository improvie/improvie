use improvie_logic::{DynAppResult, model::settings::AppSettings};

#[async_trait::async_trait]
pub trait SettingsRepository {
    type DbConnection<'a>: crate::persistence::db::DbConnection<'a>;

    async fn get_app_settings(
        &self,
        conn: Self::DbConnection<'_>,
    ) -> DynAppResult<Option<AppSettings>>;
    async fn set_app_settings(
        &self,
        conn: Self::DbConnection<'_>,
        settings: AppSettings,
    ) -> DynAppResult<()>;
}
