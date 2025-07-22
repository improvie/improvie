use crate::repository::{
    items::ItemsRepository, plays::PlaystsRepository, recents::RecentsRepository,
    rules::RulesRepository, settings::SettingsRepository,
};

macros::def_module!(RepositoriesModule {
    settings_repository: SettingsRepository,
    items_repository: ItemsRepository<for DbConnection>,
    playsts_repository: PlaystsRepository<for DbConnection>,
    rules_repository: RulesRepository,
    recents_repository: RecentsRepository<for DbConnection>,
}, {
    fn pool(&self) -> Self::DbPool;
    fn connection<'a>(&'a self) -> Self::DbConnection<'a>;
    async fn begin(&self) -> improvie_logic::DynAppResult<Self::DbTx>;
    fn record_not_found(&self) -> improvie_logic::BoxDynAppError;
});

mod macros {
    macro_rules! def_module {
        (
            $module:ident {
                $($variable:ident: $repository:ident$(<$for_ident:ident $tx:ident>)?,)*
            }, {$($tt:tt)*}
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

                $($tt)*

            }
        };
    }

    pub(super) use def_module;
}
