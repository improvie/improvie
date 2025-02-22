use std::sync::Arc;

use crate::{
    persistence::db::DbPool,
    repository::{
        health_check::HealthCheckRepositoryImpl, items::ItemsRepositoryImpl,
        playlists::PlaylistsRepositoryImpl,
    },
};

pub struct RepositoriesModuleImpl(Arc<RepositoriesModuleImplInner>);

impl Clone for RepositoriesModuleImpl {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl RepositoriesModuleImpl {
    pub fn new(db: DbPool) -> Self {
        Self(Arc::new(RepositoriesModuleImplInner::new(db)))
    }
}

macros::def_repositories_module!(
    RepositoriesModuleImpl,
    struct RepositoriesModuleImplInner {
        health_check_repository: HealthCheckRepositoryImpl = HealthCheckRepository,
        items_repository: ItemsRepositoryImpl = ItemsRepository,
        playlists_repository: PlaylistsRepositoryImpl = PlaylistsRepository,
    }
);

mod macros {

    macro_rules! def_repositories_module {
    (
        $module:ident, $struct:ident $name:ident
        { $($variable:ident: $impl:ident = $repository:ident,)* }
    ) => {
        $struct $name {
            $($variable: $impl,)*
        }

        impl $name {
            fn new(db: DbPool) -> Self {
                Self {
                    $($variable: $impl::new(db.clone()),)*
                }
            }
        }

        impl improvie_domain::modules::RepositoriesModule for $module {
            $(
                type $repository = $impl;
                fn $variable(&self) -> &Self::$repository {
                    &self.0.$variable
                }
            )*
        }

        };
    }

    pub(super) use def_repositories_module;
}
