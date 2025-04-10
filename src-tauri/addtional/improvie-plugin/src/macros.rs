#[macro_export]
macro_rules! init_log {
    () => {{
        #[allow(clippy::unwrap_used)]
        if let Some((logger, level_filter)) = $crate::LOGGER.get() {
            let _ = log::set_logger(logger);
            log::set_max_level(*level_filter);
        }
    }};
}

#[macro_export]
macro_rules! metadata {
    () => {
        #[unsafe(no_mangle)]
        pub static METADATA: $crate::PluginMetadata = $crate::PluginMetadata {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
            authors: option_env!("CARGO_PKG_AUTHORS"),
            description: option_env!("CARGO_PKG_DESCRIPTION"),
            repository: option_env!("CARGO_PKG_REPOSITORY"),
        };
    };
}

#[macro_export]
macro_rules! plugin {
    ($new:expr) => {
        #[unsafe(no_mangle)]
        pub fn plugin() -> Box<dyn $crate::Plugin> {
            Box::new($new)
        }
    };
}
