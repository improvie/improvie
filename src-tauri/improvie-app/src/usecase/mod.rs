pub mod items;
pub mod plays;
pub mod rules;
pub mod settings;

macro_rules! def_use_case {
    ($ident:ident) => {
        pub struct $ident<R: RepositoriesModule> {
            repository: &'static R,
        }

        impl<R: RepositoriesModule> $ident<R> {
            pub fn new(repository: &'static R) -> Self {
                Self { repository }
            }
        }
    };
}

use def_use_case;
