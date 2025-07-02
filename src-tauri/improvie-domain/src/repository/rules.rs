use improvie_logic::DynAppResult;
use uid::Uid;

use crate::model::rules::RuleData;

#[async_trait::async_trait]
pub trait RulesRepository {
    async fn get_rules(&self, playlist_id: Uid) -> DynAppResult<Vec<RuleData>>;

    async fn update_rules(&self, playlist_id: Uid, rules: Vec<RuleData>) -> DynAppResult<()>;
}
