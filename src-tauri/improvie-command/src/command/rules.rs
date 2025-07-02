use improvie_domain::model::rules::RuleData;
use improvie_logic::DynAppResult;
use uid::Uid;

use crate::{
    model::rules::{Rule, RuleFormat, RuleFormatIter},
    state::TauriAppState,
};

#[tauri::command]
pub async fn get_rules(state: TauriAppState<'_>, playlist_id: Uid) -> DynAppResult<Vec<Rule>> {
    let data_rules = state
        .modules
        .rules_use_case()
        .get_rules(playlist_id)
        .await?;
    Ok(data_rules.into_iter().map(Rule::from_data).collect())
}

#[tauri::command]
pub async fn update_rules(
    state: TauriAppState<'_>,
    playlist_id: Uid,
    rules: Vec<RuleData>,
) -> DynAppResult<()> {
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

#[tauri::command]
pub async fn get_first_rule_format(rules: Vec<Rule>) -> Option<RuleFormat> {
    rules.iter().find_map(|rule| rule.first())
}
