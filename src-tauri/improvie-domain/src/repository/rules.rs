use improvie_logic::{AppResult, logic::rule::Rule};
use uid::Uid;

#[async_trait::async_trait]
pub trait RulesRepository {
    async fn get_rules(&self, playlist_id: Uid) -> AppResult<Vec<Rule>>;

    async fn update_rules(&self, playlist_id: Uid, rules: Vec<Rule>) -> AppResult<()>;
}
