pub mod content;

mod macros;
use macros::*;

def_constant_enum!(
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
    pub enum Visibility {
        Normal = 0,
        Favorite = 1,
    }
);
