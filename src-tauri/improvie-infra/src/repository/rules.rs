use improvie_domain::repository::rules::RulesRepository;
use improvie_logic::{AppResult, logic::rule::RuleGeneratorRaw};
use sqlx::types::Json;

super::def_repository_impl!(RulesRepositoryImpl);

#[async_trait::async_trait]
impl RulesRepository for RulesRepositoryImpl {
    async fn get_rules(&self, playlist_id: uid::Uid) -> AppResult<Vec<RuleGeneratorRaw>> {
        let row = sqlx::query_scalar::<_, Json<Vec<RuleGeneratorRaw>>>(
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

    async fn update_rules(
        &self,
        playlist_id: uid::Uid,
        rules: Vec<RuleGeneratorRaw>,
    ) -> AppResult<()> {
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
