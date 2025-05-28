use crate::PluginLoadError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PluginMetadata {
    pub name: &'static str,
    pub version: &'static str,
    pub authors: Option<&'static str>,
    pub description: Option<&'static str>,
    pub repository: Option<&'static str>,
}

impl PluginMetadata {
    pub fn to_valid(&self) -> Result<PluginMetadata, PluginLoadError> {
        fn require_str(
            s: &'static str,
            field_name: &'static str,
            plugin_name: &str,
        ) -> Result<&'static str, PluginLoadError> {
            let s = s.trim();
            if s.is_empty() {
                Err(PluginLoadError::InvalidMetadata {
                    plugin_name: plugin_name.to_string(),
                    field_name,
                    filed_value: s.to_string(),
                })
            } else {
                Ok(s)
            }
        }

        fn filter_empty(s: Option<&'static str>) -> Option<&'static str> {
            s.and_then(|s| {
                let s = s.trim();
                if s.is_empty() { None } else { Some(s) }
            })
        }

        Ok(PluginMetadata {
            name: require_str(self.name, "name", self.name)?,
            version: require_str(self.version, "version", self.name)?,
            authors: filter_empty(self.authors),
            description: filter_empty(self.description),
            repository: filter_empty(self.repository),
        })
    }
}
