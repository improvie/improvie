use crate::usecase::{
    halth_check::HealthCheckUseCase, items::ItemsUseCase, playlists::PlaylistsUseCase,
};
use improvie_infra::{modules::RepositoriesModuleImpl, persistence::db::DbPool};

macros::def_modules!(
    RepositoriesModuleImpl,
    pub struct Modules {
        health_check_use_case: HealthCheckUseCase,
        items_use_case: ItemsUseCase,
        playlists_use_case: PlaylistsUseCase,
    }
);

mod macros {
    macro_rules! def_modules {
    (
        $repository:ident, $pub:ident $struct:ident $name:ident
        { $($variable:ident: $usecase:ident,)* }
    ) => {
        $pub $struct $name {
            $($variable: $usecase<$repository>,)*
        }

        impl $name {
            $pub fn new(db: DbPool) -> Self {
                let repository = $repository::new(db);
                Self {
                    $($variable: $usecase::new(repository.clone()),)*
                }
            }

            $(
                $pub fn $variable(&self) -> &$usecase<$repository> {
                    &self.$variable
                }
            )*
        }
    };
}

    pub(super) use def_modules;
}
