use improvie_domain::modules::RepositoriesModule;

use crate::{
    persistence::db::DbPool,
    repository::{
        items::ItemsRepositoryImpl, plays::PlaylistsRepositoryImpl, rules::RulesRepositoryImpl,
        settings::SettingsRepositoryImpl,
    },
};

async fn begin(db: &DbPool) -> improvie_logic::AppResult<crate::persistence::db::DbTx> {
    let result = db.begin();
    match result.await {
        Ok(tx) => Ok(tx),
        Err(e) => Err(improvie_logic::AppError::from(e)),
    }
}

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

        impl $trait for $name {
            type DbTx = crate::persistence::db::DbTx;
            $(
                type $repository = $impl;
                fn $variable(&self) -> &Self::$repository {
                    &self.$variable
                }
            )*

            fn begin(&self) -> impl Future<Output = improvie_logic::AppResult<Self::DbTx>> {
                begin(&self.db)
            }
        }

        };
    }

    pub(super) use def_repositories_module;
}
