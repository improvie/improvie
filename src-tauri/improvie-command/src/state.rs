use std::path::PathBuf;

use improvie_infra::{
    modules::RepositoriesModuleImpl,
    persistence::db::{DbPool, InitDbError},
};
use tauri::State;

use crate::modules::Modules;

pub struct AppState {
    pub modules: Modules,
    pub data_dir: PathBuf,
    pub document_dir: PathBuf,
}

impl AppState {
    pub async fn new(data_dir: PathBuf) -> Result<Self, InitDbError> {
        let db = DbPool::new(data_dir.clone()).await?;
        let repository = RepositoriesModuleImpl::new(db);
        let repository = std::sync::Arc::new(repository);

        let modules = Modules::new_with_repository(repository);

        let document_dir = data_dir.join("documents");
        if !document_dir.exists() {
            std::fs::create_dir_all(&document_dir)?;
        }

        Ok(Self {
            modules,
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
