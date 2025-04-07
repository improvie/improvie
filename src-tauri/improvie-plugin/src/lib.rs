use std::sync::OnceLock;

mod context;
mod metadata;
pub use context::*;
pub use metadata::*;

mod macros;

pub static LOGGER: OnceLock<(&'static dyn log::Log, log::LevelFilter)> = OnceLock::new();

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
pub type BoxResult<T> = std::result::Result<T, BoxError>;

#[allow(unused_variables)]
#[async_trait::async_trait]
pub trait Plugin: Send + Sync + 'static {
    async fn on_load(&mut self, app: &AppContext) -> BoxResult<()> {
        Ok(())
    }

    async fn on_unload(&mut self, app: &AppContext) -> BoxResult<()> {
        Ok(())
    }
}
