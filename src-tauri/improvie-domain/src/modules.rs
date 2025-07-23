use crate::repository::{
    items::ItemsRepository, plays::PlaystsRepository, recents::RecentsRepository,
    rules::RulesRepository, settings::SettingsRepository,
};

macros::def_module!(
    RepositoriesModule {
        settings_repository: SettingsRepository,
        items_repository: ItemsRepository,
        playsts_repository: PlaystsRepository,
        rules_repository: RulesRepository,
        recents_repository: RecentsRepository,
    },
    {
        fn pool(&self) -> Self::DbPool;
        fn connection<'a>(&'a self) -> Self::DbConnection<'a>;
        async fn begin(&self) -> improvie_logic::DynAppResult<Self::DbTx>;
        fn record_not_found(&self, message: String) -> improvie_logic::BoxDynAppError;
    }
);

mod macros {
    macro_rules! def_module {
        (
            $module:ident {
                $($variable:ident: $repository:ident,)*
            }, {$($tt:tt)*}
        ) => {
            #[async_trait::async_trait]
            pub trait $module: Send + Sync + Sized + 'static {
                type DbConnection<'a>: $crate::persistence::db::DbConnection<'a>;
                type DbPool: for<'a> $crate::persistence::db::DbPool<DbConnection<'a> = Self::DbConnection<'a>, DbTx = Self::DbTx>;
                type DbTx: for<'a> $crate::persistence::db::DbTx<DbConnection<'a> = Self::DbConnection<'a>, DbPool = Self::DbPool>;

                $(
                    type $repository: for<'a> $repository<DbConnection<'a> = Self::DbConnection<'a>>;
                )*

                $(
                    fn $variable(&self) -> &Self::$repository;
                )*

                $($tt)*

            }
        };
    }

    pub(super) use def_module;
}
