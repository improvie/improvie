use improvie_logic::DynAppResult;
use uid::Uid;

use crate::model::rules::RuleData;

#[async_trait::async_trait]
pub trait RulesRepository {
    type DbConnection<'a>: crate::persistence::db::DbConnection<'a>;

    async fn get_rules(&self, playlist_id: Uid) -> DynAppResult<Vec<RuleData>>;

    async fn update_rules(&self, playlist_id: Uid, rules: Vec<RuleData>) -> DynAppResult<()>;
}
