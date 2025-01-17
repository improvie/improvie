pub mod content;

mod macros;
use macros::*;

def_constant_enum!(
    pub enum Visibility {
        Normal = 0,
        Favorite = 1,
    }
);
