use improvie_app::usecase::{
    items::ItemsUseCase, plays::PlaysUseCase, recents::RecentsUseCase, rules::RulesUseCase,
    settings::SettingsUseCase,
};
use improvie_infra::modules::RepositoriesModuleImpl;

macros::def_modules!(
    RepositoriesModuleImpl,
    Modules {
        settings_use_case: SettingsUseCase,
        items_use_case: ItemsUseCase,
        plays_use_case: PlaysUseCase,
        rules_use_case: RulesUseCase,
        recents_use_case: RecentsUseCase,
    }
);

mod macros {
    macro_rules! def_modules {
    (
        $repository:ident, $name:ident
        { $($variable:ident: $usecase:ident,)* }
    ) => {
        pub struct $name {
            $($variable: $usecase<$repository>,)*
        }

        impl $name {
            pub fn new_with_repository(repository: std::sync::Arc<$repository>) -> Self {
                Self {
                    $($variable: $usecase::new(std::sync::Arc::clone(&repository)),)*
                }
            }

            $(
                pub fn $variable(&self) -> &$usecase<$repository> {
                    &self.$variable
                }
            )*
        }
    };
}

    pub(super) use def_modules;
}
