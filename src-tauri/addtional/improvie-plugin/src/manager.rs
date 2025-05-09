use std::path::PathBuf;

use crate::{BoxResult, Plugin, PluginContext, PluginFeature, PluginMetadata, theme::ThemeFeature};

pub struct PluginData {
    pub metadata: PluginMetadata,
    pub instance: Box<dyn Plugin>,
    pub is_loaded: bool,
    pub features: Vec<PluginFeature>,
}

impl PluginData {
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
            log::info!("Loading plugin {name}");
            if let Err(err) = self.load_plugin(&entry.path()).await {
                log::error!("Failed load plugin {name}: {err}");
            } else {
                log::info!("Plugin {name} loaded");
            }
        }

        Ok(())
    }

    pub async fn load_plugin(&mut self, path: &PathBuf) -> BoxResult<()> {
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

    pub async fn register_plugin(
        &mut self,
        metadata: PluginMetadata,
        mut instance: Box<dyn Plugin>,
    ) {
        let context = PluginContext::new(metadata.clone());

        let on_load = instance.on_load(&context).await;

        let is_loaded = on_load.is_ok();

        let features = match on_load {
            Ok(ok) => ok,
            Err(e) => {
                log::error!("Error plugin on load: {e}");
                vec![]
            }
        };

        self.plugins.push(PluginData {
            metadata,
            instance,
            is_loaded,
            features,
        });
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

    fn loaded_plugin(&self) -> impl Iterator<Item = &PluginData> {
        self.plugins.iter().filter(|data| data.is_loaded)
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
