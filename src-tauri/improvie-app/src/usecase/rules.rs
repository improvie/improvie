use improvie_domain::{modules::RepositoriesModule, repository::rules::RulesRepository};
use improvie_logic::{DynAppResult, logic::rule::Rule};
use uid::Uid;

super::def_use_case!(RulesUseCase);

impl<R: RepositoriesModule> RulesUseCase<R> {
    pub async fn get_rules(&self, playlist_id: Uid) -> DynAppResult<Vec<Rule>> {
        self.repository
            .rules_repository()
            .get_rules(playlist_id)
            .await
    }

    pub async fn update_rules(&self, playlist_id: Uid, rules: Vec<Rule>) -> DynAppResult<()> {
        self.repository
            .rules_repository()
            .update_rules(playlist_id, rules)
            .await
    }
}
