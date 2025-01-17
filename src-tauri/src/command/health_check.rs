use improvie_logic::AppResult;
use tauri::State;

use crate::modules::Modules;

#[tauri::command]
pub async fn health_check(modules: State<'_, Modules>) -> AppResult<bool> {
    modules.health_check_use_case().health_check().await
}
