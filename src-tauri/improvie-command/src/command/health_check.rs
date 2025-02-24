use improvie_logic::AppResult;

use crate::state::TauriAppState;

#[tauri::command]
pub async fn health_check(state: TauriAppState<'_>) -> AppResult<bool> {
    state.modules.health_check_use_case().health_check().await
}
