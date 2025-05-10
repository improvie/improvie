pub mod settings;
pub mod items;
pub mod plays;
pub mod rules;

macro_rules! def_use_case {
    ($ident:ident) => {
        pub struct $ident<R: RepositoriesModule> {
            repository: std::sync::Arc<R>,
        }

        impl<R: RepositoriesModule> $ident<R> {
            pub fn new(repository: std::sync::Arc<R>) -> Self {
                Self { repository }
            }
        }
    };
}

use def_use_case;
