use std::{path::PathBuf, sync::Arc};

use improvie_infra::persistence::db::InitDbError;
use improvie_plugin::PluginManager;
use reqwest::Client;
use tauri::{State, async_runtime::Mutex};

use crate::modules::Modules;

#[cfg(test)]
pub type AppRuntime = tauri::test::MockRuntime;
#[cfg(not(test))]
pub type AppRuntime = tauri::Wry;

pub struct AppState {
    pub modules: Arc<Modules>,
    pub pm: Arc<Mutex<PluginManager>>,
    pub data_dir: PathBuf,
    pub client: Client,
}

impl AppState {
    pub async fn new(data_dir: PathBuf) -> Result<Self, InitDbError> {
        let modules = Modules::new(data_dir.clone()).await?;

        log::info!("Start loading plugins");
        let mut pm = PluginManager::new(data_dir.clone());
        pm.register_plugin(
            improvie_builtin::METADATA.clone(),
            Box::new(improvie_builtin::BuiltinPlugin::new()),
        )
        .await;
        let _ = pm.load_plugins().await;
        let pm = Arc::new(Mutex::new(pm));
        log::info!("Plugins loaded");

        Ok(Self {
            modules: Arc::new(modules),
            pm,
            data_dir,
            client: Client::new(),
        })
    }
}

pub type TauriAppState<'a> = State<'a, AppState>;

#[cfg(test)]
pub mod tests {
    use tauri::{Manager, test::MockRuntime};

    use super::{AppState, TauriAppState};

    pub struct MockApp {
        app: tauri::App<MockRuntime>,
    }

    impl MockApp {
        pub async fn new() -> Self {
            let app = tauri::test::mock_builder()
                .build(tauri::test::mock_context(tauri::test::noop_assets()))
                .unwrap();

            let state = AppState::new(test_dir()).await.unwrap();
            app.manage(state);
            Self { app }
        }

        pub fn get_state(&self) -> TauriAppState<'_> {
            self.app.state::<AppState>()
        }
    }

    pub fn test_dir() -> std::path::PathBuf {
        std::path::Path::new(std::env!("CARGO_MANIFEST_DIR"))
            .join("test")
            .to_path_buf()
    }
}
