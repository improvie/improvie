#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PluginMetadata<'s> {
    pub name: &'s str,
    pub version: &'s str,
    pub authors: &'s str,
    pub description: &'s str,
}
