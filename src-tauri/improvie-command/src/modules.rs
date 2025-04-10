use std::path::PathBuf;

use improvie_app::usecase::{
    halth_check::HealthCheckUseCase, items::ItemsUseCase, plays::PlaysUseCase, rules::RulesUseCase,
};
use improvie_infra::{
    modules::RepositoriesModuleImpl,
    persistence::db::{DbPool, InitDbError},
};

macros::def_modules!(
    RepositoriesModuleImpl,
    pub struct Modules {
        health_check_use_case: HealthCheckUseCase,
        items_use_case: ItemsUseCase,
        plays_use_case: PlaysUseCase,
        rules_use_case: RulesUseCase,
    }
);

impl Modules {
    pub async fn new(data_dir: PathBuf) -> Result<Self, InitDbError> {
        let db = DbPool::new(data_dir).await?;
        let modules = Self::new_with_db(db);
        Ok(modules)
    }
}

mod macros {
    macro_rules! def_modules {
    (
        $repository:ident, $pub:ident $struct:ident $name:ident
        { $($variable:ident: $usecase:ident,)* }
    ) => {
        $pub $struct $name {
            $($variable: $usecase<$repository>,)*
        }

        impl $name {
            $pub fn new_with_db(db: DbPool) -> Self {
                let repository = $repository::new(db);
                Self {
                    $($variable: $usecase::new(repository.clone()),)*
                }
            }

            $(
                $pub fn $variable(&self) -> &$usecase<$repository> {
                    &self.$variable
                }
            )*
        }
    };
}

    pub(super) use def_modules;
}
