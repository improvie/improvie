use crate::repository::{
    items::ItemsRepository, plays::PlaystsRepository, rules::RulesRepository,
    settings::SettingsRepository,
};

macros::def_module!(RepositoriesModule {
    settings_repository: SettingsRepository,
    items_repository: ItemsRepository,
    playsts_repository: PlaystsRepository,
    rules_repository: RulesRepository,
});

mod macros {
    macro_rules! def_module {
        (
            $module:ident {
                $($variable:ident: $repository:ident,)*
            }
        ) => {
            pub trait $module: Clone + Send + Sync + Sized + 'static {
                $(
                    type $repository: $repository;
                )*

                $(
                    fn $variable(&self) -> &Self::$repository;
                )*
            }
        };
    }

    pub(super) use def_module;
}
