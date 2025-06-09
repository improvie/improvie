use improvie_domain::modules::RepositoriesModule;

use crate::{
    persistence::db::DbPool,
    repository::{
        items::ItemsRepositoryImpl, plays::PlaylistsRepositoryImpl, rules::RulesRepositoryImpl,
        settings::SettingsRepositoryImpl,
    },
};

macros::def_repositories_module!(
    RepositoriesModule,
    RepositoriesModuleImpl {
        settings_repository: SettingsRepositoryImpl = SettingsRepository,
        items_repository: ItemsRepositoryImpl = ItemsRepository,
        playsts_repository: PlaylistsRepositoryImpl = PlaystsRepository,
        rules_repository: RulesRepositoryImpl = RulesRepository,
    }
);

mod macros {

    macro_rules! def_repositories_module {
    (
        $trait:ident, $name:ident
        { $($variable:ident: $impl:ident = $repository:ident,)* }
    ) => {
        pub struct $name {
            db : DbPool,
            $($variable: $impl,)*
        }

        impl $name {
            pub fn new(db: DbPool) -> Self {
                Self {
                    $($variable: $impl::new(db.clone()),)*
                    db,
                }
            }
        }

        #[async_trait::async_trait]
        impl $trait for $name {
            type DbConnection<'a> = $crate::persistence::db::DbConnection<'a>;
            type DbPool = $crate::persistence::db::DbPool;
            type DbTx = $crate::persistence::db::DbTx;

            $(
                type $repository = $impl;
                fn $variable(&self) -> &Self::$repository {
                    &self.$variable
                }
            )*

            fn pool(&self) -> Self::DbPool {
                self.db.clone()
            }

            async fn begin(&self) -> improvie_logic::DynAppResult<Self::DbTx> {
                self.db.begin().await
            }
        }

        };
    }

    pub(super) use def_repositories_module;
}
