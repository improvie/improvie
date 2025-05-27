use improvie_logic::{AppResult, logic::rule::RuleGeneratorRaw};
use uid::Uid;

#[async_trait::async_trait]
pub trait RulesRepository {
    async fn get_rules(&self, playlist_id: Uid) -> AppResult<Vec<RuleGeneratorRaw>>;

    async fn update_rules(&self, playlist_id: Uid, rules: Vec<RuleGeneratorRaw>) -> AppResult<()>;
}
