use improvie_logic::{
    AppResult,
    logic::rule::{Rule, RuleFormat, RuleFormatIter},
};
use uid::Uid;

use crate::state::TauriAppState;

#[tauri::command]
pub async fn get_rules(state: TauriAppState<'_>, playlist_id: Uid) -> AppResult<Vec<Rule>> {
    state.modules.rules_use_case().get_rules(playlist_id).await
}

#[tauri::command]
pub async fn update_rules(
    state: TauriAppState<'_>,
    playlist_id: Uid,
    rules: Vec<Rule>,
) -> AppResult<()> {
    state
        .modules
        .rules_use_case()
        .update_rules(playlist_id, rules)
        .await
}

#[tauri::command]
pub async fn get_rules_format(rules: Vec<Rule>) -> Vec<RuleFormat> {
    rules.iter().flat_map(|rule| rule.formats()).collect()
}
