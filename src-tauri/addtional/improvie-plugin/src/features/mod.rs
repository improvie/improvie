use theme::ThemeFeature;

pub mod theme;

pub enum PluginFeature {
    Theme(ThemeFeature<'static>),
}
