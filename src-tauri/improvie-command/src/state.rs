use std::sync::Arc;

use tauri::State;

use crate::modules::Modules;

cfg_if::cfg_if!(
    if #[cfg(test)] {
        pub type AppRuntime = tauri::test::MockRuntime;
    } else {
        pub type AppRuntime = tauri::Wry;
    }
);

pub struct AppState {
    pub modules: Arc<Modules>,
}

pub type TauriAppState<'a> = State<'a, AppState>;

#[cfg(test)]
pub mod tests {
    use std::sync::Arc;

    use improvie_infra::persistence::db::DbPool;
    use tauri::{Manager, test::MockRuntime};

    use crate::modules::Modules;

    use super::{AppState, TauriAppState};

    pub struct MockApp {
        app: tauri::App<MockRuntime>,
    }

    impl MockApp {
        pub async fn new() -> Self {
            let app = tauri::test::mock_builder()
                .build(tauri::test::mock_context(tauri::test::noop_assets()))
                .unwrap();

            let modules = Modules::new(DbPool::new(test_dir()).await.unwrap());
            let modules = Arc::new(modules);

            let state = AppState { modules };
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
