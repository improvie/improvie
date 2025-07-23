use improvie_domain::repository::settings::SettingsRepository;
use improvie_logic::model::settings::AppSettings;
use sea_orm::ActiveValue::*;
use sea_orm::ColumnTrait;
use sea_orm::{EntityTrait, QueryFilter, QuerySelect};
use uid::Uid;

use super::def_repository_impl;

def_repository_impl!(SettingsRepositoryImpl);

#[async_trait::async_trait]
impl SettingsRepository for SettingsRepositoryImpl {
    type DbConnection<'a> = crate::persistence::db::DbConnectionImpl<'a>;

    async fn get_app_settings(
        &self,
        conn: Self::DbConnection<'_>,
    ) -> improvie_logic::DynAppResult<Option<AppSettings>> {
        improvie_row::app_settings::Entity::find()
            .select_only()
            .filter(improvie_row::app_settings::Column::Id.eq(Uid::nil()))
            .column(improvie_row::app_settings::Column::Settings)
            .into_tuple::<AppSettings>()
            .one(&conn)
            .await
            .map_err(Into::into)
    }

    async fn set_app_settings(
        &self,
        conn: Self::DbConnection<'_>,
        settings: AppSettings,
    ) -> improvie_logic::DynAppResult<()> {
        let row = improvie_row::app_settings::ActiveModel {
            settings: Set(settings),
            ..Default::default()
        };

        improvie_row::app_settings::Entity::update_many()
            .set(row)
            .filter(improvie_row::app_settings::Column::Id.eq(Uid::nil()))
            .exec(&conn)
            .await?;

        Ok(())
    }
}
