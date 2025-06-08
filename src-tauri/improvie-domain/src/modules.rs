use crate::repository::{
    items::ItemsRepository, plays::PlaystsRepository, rules::RulesRepository,
    settings::SettingsRepository,
};

macros::def_module!(RepositoriesModule {
    settings_repository: SettingsRepository,
    items_repository: ItemsRepository<DbTx>,
    playsts_repository: PlaystsRepository<DbTx>,
    rules_repository: RulesRepository,
});

mod macros {
    macro_rules! def_module {
        (
            $module:ident {
                $($variable:ident: $repository:ident$(<$tx:ident>)?,)*
            }
        ) => {
            pub trait $module: Send + Sync + Sized + 'static {
                type DbTx: crate::persistence::db::DbTx;

                $(
                    type $repository: $repository $(<$tx = Self::$tx>)?;
                )*

                $(
                    fn $variable(&self) -> &Self::$repository;
                )*
            }
        };
    }

    pub(super) use def_module;
}
