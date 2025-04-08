use crate::PluginMetadata;

pub struct PluginContext {
    metadata: PluginMetadata<'static>,
}

impl PluginContext {
    pub fn new(metadata: PluginMetadata<'static>) -> Self {
        Self { metadata }
    }

    pub fn metadata(&self) -> &PluginMetadata<'static> {
        &self.metadata
    }
}
