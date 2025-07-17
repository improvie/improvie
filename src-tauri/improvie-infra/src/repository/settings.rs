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
    async fn get_app_settings(&self) -> improvie_logic::DynAppResult<Option<AppSettings>> {
        improvie_row::app_settings::Entity::find()
            .filter(improvie_row::app_settings::Column::Id.eq(Uid::nil()))
            .select_only()
            .column(improvie_row::app_settings::Column::Settings)
            .into_tuple::<AppSettings>()
            .one(self.db.pool())
            .await
            .map_err(Into::into)
    }

    async fn set_app_settings(&self, settings: AppSettings) -> improvie_logic::DynAppResult<()> {
        let row = improvie_row::app_settings::ActiveModel {
            id: Set(Uid::nil()),
            settings: Set(settings),
            ..Default::default()
        };

        improvie_row::app_settings::Entity::update(row)
            .exec(self.db.pool())
            .await?;

        Ok(())
    }
}
