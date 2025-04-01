use improvie_domain::{modules::RepositoriesModule, repository::rules::RulesRepository};
use improvie_logic::{AppResult, logic::rule::Rule};
use uuid::Uuid;

super::def_use_case!(RulesUseCase);

impl<R: RepositoriesModule> RulesUseCase<R> {
    pub async fn get_rules(&self, playlist_id: Uuid) -> AppResult<Vec<Rule>> {
        self.repository
            .rules_repository()
            .get_rules(playlist_id)
            .await
    }

    pub async fn update_rules(&self, playlist_id: Uuid, rules: Vec<Rule>) -> AppResult<()> {
        self.repository
            .rules_repository()
            .update_rules(playlist_id, rules)
            .await
    }
}
