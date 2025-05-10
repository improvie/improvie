use crate::PluginMetadata;

pub struct PluginContext {
    metadata: PluginMetadata,
}

impl PluginContext {
    pub fn new(metadata: PluginMetadata) -> Self {
        Self { metadata }
    }

    pub fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }
}
