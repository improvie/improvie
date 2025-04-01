use std::sync::Arc;

use improvie_domain::modules::RepositoriesModule;

use crate::{
    persistence::db::DbPool,
    repository::{
        health_check::HealthCheckRepositoryImpl, items::ItemsRepositoryImpl,
        plays::PlaylistsRepositoryImpl, rules::RulesRepositoryImpl,
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
    RepositoriesModule,
    RepositoriesModuleImpl,
    struct RepositoriesModuleImplInner {
        health_check_repository: HealthCheckRepositoryImpl = HealthCheckRepository,
        items_repository: ItemsRepositoryImpl = ItemsRepository,
        playsts_repository: PlaylistsRepositoryImpl = PlaystsRepository,
        rules_repository: RulesRepositoryImpl = RulesRepository,
    }
);

mod macros {

    macro_rules! def_repositories_module {
    (
        $trait:ident, $module:ident, $struct:ident $name:ident
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

        impl $trait for $module {
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
