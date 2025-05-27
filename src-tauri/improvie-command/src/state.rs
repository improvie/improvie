use std::path::PathBuf;

use improvie_infra::{
    modules::RepositoriesModuleImpl,
    persistence::db::{DbPool, InitDbError},
};
use improvie_plugin::PluginManager;
use tauri::{State, async_runtime::Mutex};

use crate::modules::Modules;

pub struct AppState {
    pub modules: Modules,
    pub pm: Mutex<PluginManager>,
    pub data_dir: PathBuf,
    pub document_dir: PathBuf,
}

impl AppState {
    pub async fn new(data_dir: PathBuf) -> Result<Self, InitDbError> {
        static REPOSITORY: std::sync::OnceLock<RepositoriesModuleImpl> = std::sync::OnceLock::new();

        let db = DbPool::new(&data_dir).await?;
        let repository = RepositoriesModuleImpl::new(db);
        let repository = REPOSITORY.get_or_init(|| repository);

        let modules = Modules::new_with_repository(repository);

        let document_dir = data_dir.join("documents");
        if !document_dir.exists() {
            std::fs::create_dir_all(&document_dir)?;
        }

        log::info!("Start loading plugins");
        let mut pm = PluginManager::new(data_dir.clone());
        pm.register_plugin(
            improvie_builtin::METADATA.clone(),
            Box::new(improvie_builtin::BuiltinPlugin::new()),
        )
        .await;
        let _ = pm.load_plugins().await;
        log::info!("Plugins loaded");

        let pm = Mutex::new(pm);

        Ok(Self {
            modules,
            pm,
            data_dir,
            document_dir,
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

            let test_dir = test_dir();
            let state = AppState::new(test_dir).await.unwrap();
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
