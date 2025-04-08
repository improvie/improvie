use std::path::PathBuf;

use crate::{BoxResult, Plugin, PluginContext, PluginMetadata};

pub struct PluginData {
    pub metadata: PluginMetadata<'static>,
    pub instance: Box<dyn Plugin>,
    pub lib: libloading::Library,
    pub is_loaded: bool,
}

impl PluginData {
    pub async fn load(path: &PathBuf) -> BoxResult<Self> {
        let lib = unsafe { libloading::Library::new(path) }?;

        let plugin_fn = unsafe { lib.get::<fn() -> Box<dyn Plugin>>(b"plugin")? };
        let metadata: &PluginMetadata =
            unsafe { &**lib.get::<*const PluginMetadata>(b"METADATA")?.clone() };

        let mut instance = plugin_fn();

        let context = PluginContext::new(metadata.clone());

        let is_loaded = match instance.on_load(&context).await {
            Ok(_) => true,
            Err(e) => {
                log::error!("Error plugin on load: {e}");
                false
            }
        };

        Ok(Self {
            metadata: metadata.clone(),
            instance: plugin_fn(),
            lib,
            is_loaded,
        })
    }

    /// Unload the plugin.
    ///
    /// Returns `true` if the plugin was unloaded successfully,
    /// `false` if it was already unloaded.
    pub async fn unload(&mut self) -> bool {
        if self.is_loaded {
            let context = PluginContext::new(self.metadata.clone());
            self.instance.on_unload(&context).await;
            self.is_loaded = false;
            true
        } else {
            false
        }
    }
}

pub struct PluginManager {
    plugins: Vec<PluginData>,
    data_dir: PathBuf,
}

impl PluginManager {
    pub const PLUGIN_DIR: &str = "plugins";

    pub fn new(data_dir: PathBuf) -> Self {
        Self {
            plugins: Vec::new(),
            data_dir,
        }
    }

    pub async fn load_plugins(&mut self) -> BoxResult<()> {
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
            if let Err(err) = self.load_plugin(&entry.path()).await {
                log::error!("Failed load plugin {name}: {err}");
            }
        }

        Ok(())
    }

    pub async fn load_plugin(&mut self, path: &PathBuf) -> BoxResult<()> {
        let plugin_data = PluginData::load(path).await?;

        self.plugins.push(plugin_data);

        Ok(())
    }

    pub async fn unload_plugin(&mut self, name: &str) -> BoxResult<()> {
        let opt = self
            .plugins
            .iter_mut()
            .find(|data| data.metadata.name == name);

        if let Some(data) = opt {
            if data.unload().await {
                Ok(())
            } else {
                Err(format!("Plugin {name} is not loaded").into())
            }
        } else {
            Err(format!("Plugin {name} not found").into())
        }
    }

    pub async fn unload_plugins(&mut self) {
        for plugin in &mut self.plugins {
            plugin.unload().await;
        }
    }
}
