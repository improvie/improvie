#[derive(Debug, thiserror::Error, more_convert::VariantName)]
#[variant_name(prefix = "Plugin")]
pub enum PluginLoadError {
    #[error("io error on plugin: {0}")]
    Io(#[from] std::io::Error),
    #[error("loading plugin failed: {0}")]
    Lib(#[from] libloading::Error),
    #[error("invalid plugin metadata: {plugin_name}'s {field_name}")]
    InvalidMetadata {
        plugin_name: String,
        field_name: &'static str,
        filed_value: String,
    },
}
