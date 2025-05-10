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
        macro_rules! require_str {
            ($var:ident) => {{
                let var = self.$var.trim();
                if var.is_empty() {
                    return Err(PluginLoadError::InvalidMetadata {
                        plugin_name: self.name.to_string(),
                        field_name: stringify!($var),
                        filed_value: var.to_string(),
                    });
                } else {
                    var
                }
            }};
        }

        fn filter_empty(s: Option<&'static str>) -> Option<&'static str> {
            s.and_then(|s| {
                let s = s.trim();
                if s.is_empty() { None } else { Some(s) }
            })
        }

        Ok(PluginMetadata {
            name: require_str!(name),
            version: require_str!(version),
            authors: filter_empty(self.authors),
            description: filter_empty(self.description),
            repository: filter_empty(self.repository),
        })
    }
}
