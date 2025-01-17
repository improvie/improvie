use improvie_app::model::content::ItemNode;
use improvie_logic::AppResult;
use tauri::State;
use uuid::Uuid;

use crate::modules::Modules;

#[tauri::command]
pub async fn get_item(modules: State<'_, Modules>, id: Option<Uuid>) -> AppResult<ItemNode> {
    let _ = modules;
    let _ = id;
    todo!()
    // modules.content_use_case().get_item(id)
}
