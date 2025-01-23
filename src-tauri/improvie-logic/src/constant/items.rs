use super::def_constant_enum;

def_constant_enum!(
    pub enum ItemKind {
        Folder = 1,
        Content = 2,
    }
);

def_constant_enum!(
    pub enum ContentKind {
        Video = 1,
        Audio = 2,
    }
);
