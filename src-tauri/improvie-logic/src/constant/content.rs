use super::def_constant_enum;

def_constant_enum!(
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
    pub enum ContentKind {
        Video = 0,
        Audio = 1,
        Markdown = 2,
    }
);

def_constant_enum!(
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
    pub enum ContentVis {
        Visible = 0,
        Favorite = 1,
    }
);
