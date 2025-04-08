#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PluginMetadata<'a> {
    pub name: &'a str,
    pub version: &'a str,
    pub authors: MaybeEmptyStr<'a>,
    pub description: MaybeEmptyStr<'a>,
    pub repository: MaybeEmptyStr<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MaybeEmptyStr<'a>(&'a str);

impl<'a> MaybeEmptyStr<'a> {
    pub const fn new(s: &'a str) -> Self {
        Self(s)
    }

    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub const fn as_str(&self) -> Option<&'a str> {
        if self.is_empty() { None } else { Some(self.0) }
    }
}
