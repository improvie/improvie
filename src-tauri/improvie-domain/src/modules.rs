use crate::repository::{
    health_check::HealthCheckRepository, items::ItemsRepository, plays::PlaystsRepository,
    rules::RulesRepository,
};

macros::def_module!(RepositoriesModule {
    health_check_repository: HealthCheckRepository,
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
