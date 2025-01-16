use super::def_constant_enum;

def_constant_enum!(
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
    pub enum ContentKind {
        Video = 0,
        Audio = 1,
        Markdown = 2,
    }
);
