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
            authors: env!("CARGO_PKG_AUTHORS"),
            description: env!("CARGO_PKG_DESCRIPTION"),
        };
    };
}
