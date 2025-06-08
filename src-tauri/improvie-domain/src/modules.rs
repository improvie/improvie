use crate::repository::{
    items::ItemsRepository, plays::PlaystsRepository, rules::RulesRepository,
    settings::SettingsRepository,
};

macros::def_module!(RepositoriesModule {
    settings_repository: SettingsRepository,
    items_repository: = for ItemsRepository<DbConnection>,
    playsts_repository: = for PlaystsRepository<DbConnection>,
    rules_repository: RulesRepository,
});

mod macros {
    macro_rules! def_module {
        (
            $module:ident {
                $($variable:ident: $( = $fo:ident )?$repository:ident$(<$tx:ident>)?,)*
            }
        ) => {
            pub trait $module: Send + Sync + Sized + 'static {
                type DbConnection<'a>: crate::persistence::db::DbConnection<'a>;

                $(
                    type $repository: $($fo<'a>)? $repository $(<$tx<'a> = Self::$tx<'a>>)?;
                )*

                $(
                    fn $variable(&self) -> &Self::$repository;
                )*

                // fn connection(&self) -> &Self::DbConnection<'_>;
            }
        };
    }

    pub(super) use def_module;
}
