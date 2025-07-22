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
pub async fn get_rules_format(
    state: TauriAppState<'_>,
    playlist_id: Uid,
) -> DynAppResult<Vec<RuleFormat>> {
    let rules = get_rules(state.clone(), playlist_id).await?;
    let mut formats = Vec::new();
    for rule in rules {
        formats.extend(rule.formats(&state).await);
    }
    Ok(formats)
}

#[tauri::command]
pub async fn get_rules_format_with_shuffle(
    state: TauriAppState<'_>,
    playlist_id: Uid,
) -> DynAppResult<Vec<RuleFormat>> {
    let mut rules = get_rules(state.clone(), playlist_id).await?;
    rand::prelude::SliceRandom::shuffle(rules.as_mut_slice(), &mut rand::rng());
    let mut formats = Vec::new();
    for rule in rules {
        formats.extend(rule.shuffle(&state).await);
    }
    Ok(formats)
}

#[tauri::command]
pub async fn get_thumbnail_content_uid(
    state: TauriAppState<'_>,
    playlist_id: Uid,
) -> DynAppResult<Option<Uid>> {
    let rules = get_rules(state.clone(), playlist_id).await?;
    if let Some(rule) = rules.first() {
        Ok(rule.thumbnail(&state).await)
    } else {
        Ok(None)
    }
}
