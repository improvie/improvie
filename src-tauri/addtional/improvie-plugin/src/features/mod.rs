mod theme;
pub use theme::ThemeFeature;

mod rule;
pub use rule::*;

pub enum PluginFeature {
    Theme(ThemeFeature<'static>),
}
