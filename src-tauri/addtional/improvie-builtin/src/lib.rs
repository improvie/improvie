use improvie_plugin::{BoxResult, Plugin, PluginContext, PluginFeature};
use themes::{black_theme, catppuccin_frappe_theme, tokyonight_night_theme, white_theme};

improvie_plugin::metadata!();

pub mod themes;

pub struct BuiltinPlugin {}

impl BuiltinPlugin {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl Plugin for BuiltinPlugin {
    async fn on_load(&mut self, _ctx: &PluginContext) -> BoxResult<Vec<PluginFeature>> {
        Ok(vec![
            PluginFeature::Theme(white_theme()),
            PluginFeature::Theme(black_theme()),
            PluginFeature::Theme(catppuccin_frappe_theme()),
            PluginFeature::Theme(tokyonight_night_theme()),
        ])
    }
}
