use std::{path::PathBuf, sync::Arc};

use improvie_infra::persistence::db::InitDbError;
use improvie_yt::YtStore;
use tauri::{State, async_runtime::RwLock};

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
    pub yt: Arc<RwLock<YtStore>>,
}

impl AppState {
    pub async fn new(data_dir: PathBuf) -> Result<Self, InitDbError> {
        let modules = Modules::new_with_db(data_dir.clone()).await?;
        let yt = Arc::new(RwLock::new(YtStore::Loading));
        let captured_yt = yt.clone();

        std::thread::spawn(move || {
            improvie_yt::YtIntegration::new_background(data_dir, captured_yt);
        });

        Ok(Self {
            modules: Arc::new(modules),
            yt,
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
