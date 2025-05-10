use improvie_logic::AppResult;
use improvie_plugin::features::ThemeFeature;

use crate::state::TauriAppState;

#[tauri::command]
pub async fn get_themes(state: TauriAppState<'_>) -> AppResult<Vec<ThemeFeature<'static>>> {
    Ok(state.pm.lock().await.get_themes())
}
