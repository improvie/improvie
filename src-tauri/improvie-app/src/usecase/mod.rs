pub mod halth_check;
pub mod items;
pub mod playlists;

macro_rules! def_use_case {
    ($ident:ident) => {
        pub struct $ident<R: RepositoriesModule> {
            repository: R,
        }

        impl<R: RepositoriesModule> $ident<R> {
            pub fn new(repository: R) -> Self {
                Self { repository }
            }
        }
    };
}

use def_use_case;
