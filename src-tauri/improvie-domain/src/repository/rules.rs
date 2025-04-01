use improvie_logic::{AppResult, logic::rule::Rule};
use uuid::Uuid;

#[async_trait::async_trait]
pub trait RulesRepository {
    async fn get_rules(&self, playlist_id: Uuid) -> AppResult<Vec<Rule>>;

    async fn update_rules(&self, playlist_id: Uuid, rules: Vec<Rule>) -> AppResult<()>;
}
