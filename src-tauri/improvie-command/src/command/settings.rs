use improvie_logic::{DynAppResult, model::settings::AppSettings};

use crate::state::TauriAppState;

#[tauri::command]
pub async fn get_app_settings(state: TauriAppState<'_>) -> DynAppResult<AppSettings> {
    state.modules.settings_use_case().get_app_settings().await
}

#[tauri::command]
pub async fn set_app_settings(state: TauriAppState<'_>, settings: AppSettings) -> DynAppResult<()> {
    state
        .modules
        .settings_use_case()
        .set_app_settings(settings)
        .await
}
