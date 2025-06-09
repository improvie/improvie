use crate::repository::{
    items::ItemsRepository, plays::PlaystsRepository, rules::RulesRepository,
    settings::SettingsRepository,
};

macros::def_module!(RepositoriesModule {
    settings_repository: SettingsRepository,
    items_repository: ItemsRepository<for DbConnection>,
    playsts_repository: PlaystsRepository<for DbConnection>,
    rules_repository: RulesRepository,
});

mod macros {
    macro_rules! def_module {
        (
            $module:ident {
                $($variable:ident: $repository:ident$(<$for_ident:ident $tx:ident>)?,)*
            }
        ) => {
            #[async_trait::async_trait]
            pub trait $module: Send + Sync + Sized + 'static {
                type DbConnection<'a>: $crate::persistence::db::DbConnection<'a>;
                type DbPool: for<'a> $crate::persistence::db::DbPool<DbConnection<'a> = Self::DbConnection<'a>, DbTx = Self::DbTx>;
                type DbTx: for<'a> $crate::persistence::db::DbTx<DbConnection<'a> = Self::DbConnection<'a>, DbPool = Self::DbPool>;

                $(
                    type $repository: $($for_ident<'a>)? $repository $(<$tx<'a> = Self::$tx<'a>>)?;
                )*

                $(
                    fn $variable(&self) -> &Self::$repository;
                )*

                fn pool(&self) -> Self::DbPool;
                async fn begin(&self) -> improvie_logic::DynAppResult<Self::DbTx>;
            }
        };
    }

    pub(super) use def_module;
}
