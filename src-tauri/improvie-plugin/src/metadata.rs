#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PluginMetadata<'s> {
    pub name: &'s str,
    pub version: &'s str,
    pub authors: &'s str,
    pub description: &'s str,
    pub repository: Option<&'s str>,
}

unsafe impl Send for PluginMetadata<'_> {}
unsafe impl Sync for PluginMetadata<'_> {}
