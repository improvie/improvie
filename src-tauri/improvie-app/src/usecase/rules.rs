use improvie_domain::{
    model::rules::RuleData, modules::RepositoriesModule, persistence::db::DbTx,
    repository::rules::RulesRepository,
};
use improvie_logic::DynAppResult;
use uid::Uid;

super::def_use_case!(RulesUseCase);

impl<R: RepositoriesModule> RulesUseCase<R> {
    pub async fn get_rules(&self, playlist_id: Uid) -> DynAppResult<Vec<RuleData>> {
        self.repository
            .rules_repository()
            .get_rules(self.repository.connection(), playlist_id)
            .await
    }

    pub async fn update_rules(&self, playlist_id: Uid, rules: Vec<RuleData>) -> DynAppResult<()> {
        let tx = self.repository.begin().await?;
        let conn = tx.connection();
        let result = self
            .repository
            .rules_repository()
            .update_rules(conn, playlist_id, rules)
            .await;

        super::tx_commit!(tx, result)
    }
}
