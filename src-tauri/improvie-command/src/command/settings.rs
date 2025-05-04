use improvie_logic::{AppResult, model::settings::AppSettings};

use crate::state::TauriAppState;

#[tauri::command]
pub async fn get_app_settings(state: TauriAppState<'_>) -> AppResult<AppSettings> {
    state.modules.settings_use_case().get_app_settings().await
}

#[tauri::command]
pub async fn set_app_settings(state: TauriAppState<'_>, settings: AppSettings) -> AppResult<()> {
    state
        .modules
        .settings_use_case()
        .set_app_settings(settings)
        .await
}
