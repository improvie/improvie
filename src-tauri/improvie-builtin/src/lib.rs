use improvie_plugin::Plugin;

improvie_plugin::metadata!();

pub struct BuiltinPlugin {}

impl BuiltinPlugin {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }
}

impl Plugin for BuiltinPlugin {}
