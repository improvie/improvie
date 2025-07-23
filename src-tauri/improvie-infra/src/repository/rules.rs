use improvie_domain::{model::rules::RuleData, repository::rules::RulesRepository};
use improvie_logic::DynAppResult;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QuerySelect};

super::def_repository_impl!(RulesRepositoryImpl);

#[async_trait::async_trait]
impl RulesRepository for RulesRepositoryImpl {
    type DbConnection<'a> = crate::persistence::db::DbConnectionImpl<'a>;

    async fn get_rules(
        &self,
        conn: Self::DbConnection<'_>,
        playlist_id: uid::Uid,
    ) -> DynAppResult<Vec<RuleData>> {
        let row = improvie_row::playlists::Entity::find()
            .filter(improvie_row::playlists::Column::ItemId.eq(playlist_id))
            .select_only()
            .column(improvie_row::playlists::Column::Rules)
            .into_tuple::<String>()
            .one(&conn)
            .await?;

        let rules_json = row.ok_or_else(|| {
            sea_orm::error::DbErr::RecordNotFound("Playlist not found".to_string())
        })?;

        let rules: Vec<RuleData> = serde_json::from_str(&rules_json)
            .map_err(|e| sea_orm::error::DbErr::Json(e.to_string()))?;

        Ok(rules)
    }

    async fn update_rules(
        &self,
        conn: Self::DbConnection<'_>,
        playlist_id: uid::Uid,
        rules: Vec<RuleData>,
    ) -> DynAppResult<()> {
        let json = serde_json::to_string(&rules)
            .map_err(|e| sea_orm::error::DbErr::Json(e.to_string()))?;

        let row = improvie_row::playlists::ActiveModel {
            rules: sea_orm::Set(json),
            ..Default::default()
        };

        improvie_row::playlists::Entity::update_many()
            .set(row)
            .filter(improvie_row::playlists::Column::ItemId.eq(playlist_id))
            .exec(&conn)
            .await?;

        Ok(())
    }
}
