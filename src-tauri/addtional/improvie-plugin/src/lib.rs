use features::PluginFeature;
use std::sync::OnceLock;

mod context;
mod error;
mod manager;
mod metadata;
pub use context::PluginContext;
pub use error::*;
pub use manager::*;
pub use metadata::PluginMetadata;

pub mod features;

mod macros;

pub static LOGGER: OnceLock<(&'static dyn log::Log, log::LevelFilter)> = OnceLock::new();

#[async_trait::async_trait]
pub trait Plugin: Send + Sync + 'static {
    async fn on_enable(
        &self,
        ctx: &PluginContext,
    ) -> Result<Vec<PluginFeature>, Box<dyn std::error::Error + Send + Sync>>;
    async fn on_disable(&self, ctx: &PluginContext);
}
