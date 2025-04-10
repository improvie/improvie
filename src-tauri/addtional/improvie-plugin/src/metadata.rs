#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PluginMetadata<'a> {
    pub name: &'a str,
    pub version: &'a str,
    pub authors: Option<&'a str>,
    pub description: Option<&'a str>,
    pub repository: Option<&'a str>,
}
