use improvie_domain::{model::rules::RuleData, repository::rules::RulesRepository};
use improvie_logic::DynAppResult;
use improvie_row::playlists::JsonRuleDataList;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QuerySelect};

super::def_repository_impl!(RulesRepositoryImpl);

#[async_trait::async_trait]
impl RulesRepository for RulesRepositoryImpl {
    async fn get_rules(&self, playlist_id: uid::Uid) -> DynAppResult<Vec<RuleData>> {
        let data = improvie_row::playlists::Entity::find()
            .filter(improvie_row::playlists::Column::ItemId.eq(playlist_id))
            .select_only()
            .column(improvie_row::playlists::Column::Rules)
            .into_tuple::<JsonRuleDataList>()
            .one(self.db.pool())
            .await?;

        Ok(data.map(|d| d.0).unwrap_or_default())
    }

    async fn update_rules(&self, playlist_id: uid::Uid, rules: Vec<RuleData>) -> DynAppResult<()> {
        let row = improvie_row::playlists::ActiveModel {
            item_id: sea_orm::Set(playlist_id),
            rules: sea_orm::Set(JsonRuleDataList(rules)),
            ..Default::default()
        };

        // TODO: update without returning the row
        improvie_row::playlists::Entity::update(row)
            .exec(self.db.pool())
            .await?;

        Ok(())
    }
}
