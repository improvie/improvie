use improvie_domain::repository::rules::RulesRepository;
use improvie_logic::{AppResult, logic::rule::Rule};
use sqlx::types::Json;

super::def_repository_impl!(RulesRepositoryImpl);

#[async_trait::async_trait]
impl RulesRepository for RulesRepositoryImpl {
    async fn get_rules(&self, playlist_id: uuid::Uuid) -> AppResult<Vec<Rule>> {
        let rows = sqlx::query_scalar::<_, Json<Rule>>(
            "
SELECT rules
FROM playlists
WHERE item_id = ?
",
        )
        .bind(playlist_id)
        .fetch_all(&self.db.pool())
        .await?;

        let rules = rows.into_iter().map(|r| r.0).collect();

        Ok(rules)
    }
}
