use std::sync::OnceLock;

mod context;
mod error;
mod features;
mod manager;
mod metadata;
pub use context::*;
pub use error::*;
pub use features::*;
pub use manager::*;
pub use metadata::*;

mod macros;

pub static LOGGER: OnceLock<(&'static dyn log::Log, log::LevelFilter)> = OnceLock::new();

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
pub type BoxResult<T> = std::result::Result<T, BoxError>;

#[async_trait::async_trait]
pub trait Plugin: Send + Sync + 'static {
    async fn on_enable(&self, ctx: &PluginContext) -> BoxResult<Vec<PluginFeature>>;
    async fn on_disable(&self, ctx: &PluginContext);
}
