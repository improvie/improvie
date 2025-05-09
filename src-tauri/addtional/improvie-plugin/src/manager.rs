use std::path::PathBuf;

use crate::{
    Plugin, PluginContext, PluginLoadError, PluginFeature, PluginMetadata, theme::ThemeFeature,
};

pub struct PluginData {
    metadata: PluginMetadata,
    instance: Box<dyn Plugin>,
    state: PluginState,
    features: Vec<PluginFeature>,
}

impl PluginData {
    pub fn state(&self) -> &PluginState {
        &self.state
    }
}

#[derive(Debug, Clone)]
pub enum PluginState {
    Loaded,
    LoadFailed(String),
    Unloaded,
}

impl PluginState {
    pub fn is_loaded(&self) -> bool {
        matches!(self, PluginState::Loaded)
    }
}

impl PluginData {
    #[must_use]
    pub async fn enable(&mut self) -> PluginState {
        match self.state {
            PluginState::Loaded => {
                log::warn!("Plugin {} is already loaded", self.metadata.name);
                PluginState::Loaded
            }
            PluginState::LoadFailed(_) | PluginState::Unloaded => {
                log::info!("Loading plugin {}", self.metadata.name);
                let context = PluginContext::new(self.metadata.clone());
                match self.instance.on_enable(&context).await {
                    Ok(features) => {
                        self.features = features;
                        self.state = PluginState::Loaded;
                        log::info!("Plugin {} loaded successfully", self.metadata.name);
                    }
                    Err(err) => {
                        self.state = PluginState::LoadFailed(err.to_string());
                        log::error!("Failed to load plugin {}: {}", self.metadata.name, err);
                    }
                }
                self.state.clone()
            }
        }
    }

    /// Unload the plugin.
    ///
    /// Returns `true` if the plugin was unloaded successfully,
    /// `false` if it was already unloaded.
    pub async fn disable(&mut self) {
        match self.state {
            PluginState::Loaded => {
                log::info!("Unloading plugin {}", self.metadata.name);
                let context = PluginContext::new(self.metadata.clone());
                self.instance.on_disable(&context).await;
                self.state = PluginState::Unloaded;
                log::info!("Plugin {} unloaded successfully", self.metadata.name);
            }
            _ => {
                log::warn!("Plugin {} is already unloaded", self.metadata.name);
            }
        }
    }
}

pub struct PluginManager {
    plugins: Vec<PluginData>,
    libs: Vec<libloading::Library>,
    data_dir: PathBuf,
}

impl PluginManager {
    pub const PLUGIN_DIR: &str = "plugins";

    pub fn new(data_dir: PathBuf) -> Self {
        Self {
            plugins: Vec::new(),
            libs: Vec::new(),
            data_dir,
        }
    }

    pub async fn load_plugins(&mut self) -> Result<(), PluginLoadError> {
        let plugins_dir = self.data_dir.join(Self::PLUGIN_DIR);
        if !plugins_dir.exists() {
            std::fs::create_dir_all(&plugins_dir)?;

            // plugins is empty, so we can skip loading
            return Ok(());
        }

        let dir_entries = std::fs::read_dir(&plugins_dir)?;

        for entry in dir_entries {
            let Ok(entry) = entry else {
                continue;
            };
            if !entry.file_type().is_ok_and(|v| v.is_file()) {
                continue;
            }
            let name = entry.file_name().to_string_lossy().to_string();
            log::info!("Loading plugin {name}");
            if let Err(err) = self.load_plugin(&entry.path()).await {
                log::error!("Failed load plugin {name}: {err}");
            } else {
                log::info!("Plugin {name} loaded");
            }
        }

        Ok(())
    }

    pub async fn load_plugin(&mut self, path: &PathBuf) -> Result<(), PluginLoadError> {
        let lib = unsafe { libloading::Library::new(path) }?;

        let plugin_fn = unsafe { lib.get::<fn() -> Box<dyn Plugin>>(b"plugin")? };
        let metadata: &PluginMetadata =
            unsafe { &**lib.get::<*const PluginMetadata>(b"METADATA")? };

        let metadata = metadata.to_valid()?;

        let instance = plugin_fn();

        self.register_plugin(metadata, instance).await;
        self.libs.push(lib);

        Ok(())
    }

    pub async fn register_plugin(&mut self, metadata: PluginMetadata, instance: Box<dyn Plugin>) {
        let context = PluginContext::new(metadata.clone());

        let on_load = instance.on_enable(&context).await;

        let (state, features) = match on_load {
            Ok(features) => (PluginState::Loaded, features),
            Err(err) => {
                log::error!("Failed to load plugin {}: {}", metadata.name, err);
                (PluginState::LoadFailed(err.to_string()), vec![])
            }
        };

        self.plugins.push(PluginData {
            metadata,
            instance,
            state,
            features,
        });
    }

    #[must_use]
    pub async fn enable_plugin(&mut self, name: &str) -> Option<PluginState> {
        let opt = self
            .plugins
            .iter_mut()
            .find(|data| data.metadata.name == name);

        if let Some(data) = opt {
            Some(data.enable().await)
        } else {
            log::warn!("Plugin {} not found", name);
            None
        }
    }

    #[must_use]
    pub async fn disable_plugin(&mut self, name: &str) -> bool {
        let opt = self
            .plugins
            .iter_mut()
            .find(|data| data.metadata.name == name);

        if let Some(data) = opt {
            data.disable().await;
            true
        } else {
            log::warn!("Plugin {} not found", name);
            false
        }
    }

    pub async fn disable_plugins(&mut self) {
        for plugin in &mut self.plugins {
            plugin.disable().await;
        }
    }

    fn loaded_plugin(&self) -> impl Iterator<Item = &PluginData> {
        self.plugins.iter().filter(|data| data.state.is_loaded())
    }

    pub fn get_loaded_plugins(&self) -> Vec<PluginMetadata> {
        self.loaded_plugin()
            .map(|data| data.metadata.clone())
            .collect()
    }

    pub fn get_themes(&self) -> Vec<ThemeFeature<'static>> {
        self.loaded_plugin()
            .flat_map(|data| {
                data.features
                    .iter()
                    .filter_map(|feature| match feature {
                        PluginFeature::Theme(theme) => Some(theme.clone()),
                        #[allow(unreachable_patterns)]
                        _ => None,
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

impl Drop for PluginManager {
    fn drop(&mut self) {
        self.plugins.clear();
        self.libs.clear();
    }
}
