use improvie_domain::modules::RepositoriesModule;

use crate::{
    persistence::db::DbPoolImpl,
    repository::{
        items::ItemsRepositoryImpl, plays::PlaylistsRepositoryImpl, recents::RecentsRepositoryImpl,
        rules::RulesRepositoryImpl, settings::SettingsRepositoryImpl,
    },
};

macros::def_repositories_module!(
    RepositoriesModule,
    RepositoriesModuleImpl {
        settings_repository: SettingsRepositoryImpl = SettingsRepository,
        items_repository: ItemsRepositoryImpl = ItemsRepository,
        playsts_repository: PlaylistsRepositoryImpl = PlaystsRepository,
        rules_repository: RulesRepositoryImpl = RulesRepository,
        recents_repository: RecentsRepositoryImpl = RecentsRepository,
    }
);

mod macros {

    macro_rules! def_repositories_module {
    (
        $trait:ident, $name:ident
        { $($variable:ident: $impl:ident = $repository:ident,)* }
    ) => {
        pub struct $name {
            db : DbPoolImpl,
            $($variable: $impl,)*
        }

        impl $name {
            pub fn new(db: DbPoolImpl) -> Self {
                Self {
                    $($variable: $impl::new(),)*
                    db,
                }
            }
        }

        #[async_trait::async_trait]
        impl $trait for $name {
            type DbConnection<'a> = $crate::persistence::db::DbConnectionImpl<'a>;
            type DbPool = $crate::persistence::db::DbPoolImpl;
            type DbTx = $crate::persistence::db::DbTxImpl;

            $(
                type $repository = $impl;
                fn $variable(&self) -> &Self::$repository {
                    &self.$variable
                }
            )*

            fn pool(&self) -> Self::DbPool {
                self.db.clone()
            }

            fn connection<'a>(&'a self) -> Self::DbConnection<'a> {
                use improvie_domain::persistence::db::DbPool;
                self.db.connection()
            }

            async fn begin(&self) -> improvie_logic::DynAppResult<Self::DbTx> {
                self.db.begin().await
            }

            fn record_not_found(&self) -> improvie_logic::BoxDynAppError {
                sea_orm::DbErr::RecordNotFound(String::from("Record not found")).into()
            }
        }

        };
    }

    pub(super) use def_repositories_module;
}
