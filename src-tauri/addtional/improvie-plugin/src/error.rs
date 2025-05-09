#[derive(Debug, thiserror::Error, more_convert::VariantName)]
#[variant_name(prefix = "Plugin")]
pub enum PluginLoadError {
    #[error("plugin not found: {0}")]
    NotFound(String),
    #[error("plugin not loaded: {0}")]
    NotLoaded(String),
    #[error("plugin already loaded: {0}")]
    AlreadyLoaded(String),
    #[error("plugin load error: {0}")]
    LoadError(String),
    #[error("plugin unload error: {0}")]
    UnloadError(String),
    #[error("invalid plugin metadata: {plugin_name}'s {field_name}")]
    InvalidMetadata {
        plugin_name: String,
        field_name: &'static str,
        filed_value: String,
    },
}
