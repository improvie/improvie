#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PluginMetadata {
    pub name: &'static str,
    pub version: &'static str,
    pub authors: Option<&'static str>,
    pub description: Option<&'static str>,
    pub repository: Option<&'static str>,
}
