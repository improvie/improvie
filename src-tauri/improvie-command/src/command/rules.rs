use improvie_logic::{AppResult, logic::rule::Rule};
use uuid::Uuid;

use crate::state::TauriAppState;

#[tauri::command]
pub async fn get_rules(state: TauriAppState<'_>, playlist_id: Uuid) -> AppResult<Vec<Rule>> {
    state.modules.rules_use_case().get_rules(playlist_id).await
}
