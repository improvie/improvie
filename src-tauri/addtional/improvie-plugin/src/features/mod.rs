mod theme;
pub use theme::ThemeFeature;

pub enum PluginFeature {
    Theme(ThemeFeature<'static>),
}
