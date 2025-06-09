use improvie_domain::repository::rules::RulesRepository;
use improvie_logic::{DynAppResult, logic::rule::Rule};
use sqlx::types::Json;

super::def_repository_impl!(RulesRepositoryImpl);

#[async_trait::async_trait]
impl RulesRepository for RulesRepositoryImpl {
    async fn get_rules(&self, playlist_id: uid::Uid) -> DynAppResult<Vec<Rule>> {
        let row = sqlx::query_scalar::<_, Json<Vec<Rule>>>(
            "
SELECT rules
FROM playlists
WHERE item_id = ?
",
        )
        .bind(playlist_id)
        .fetch_one(&self.db.pool())
        .await?;

        Ok(row.0)
    }

    async fn update_rules(&self, playlist_id: uid::Uid, rules: Vec<Rule>) -> DynAppResult<()> {
        sqlx::query(
            "
UPDATE playlists
SET rules = ?
WHERE item_id = ?
",
        )
        .bind(Json(rules))
        .bind(playlist_id)
        .execute(&self.db.pool())
        .await?;

        Ok(())
    }
}
