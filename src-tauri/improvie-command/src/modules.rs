use std::{path::PathBuf, sync::Arc};

use improvie_app::usecase::{
    halth_check::HealthCheckUseCase, items::ItemsUseCase, plays::PlaystsUseCase,
    rules::RulesUseCase,
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
        playsts_use_case: PlaystsUseCase,
        rules_use_case: RulesUseCase,
    }
);

impl Modules {
    pub async fn new_with_db(data_dir: PathBuf) -> Result<Arc<Self>, InitDbError> {
        let db = DbPool::new(data_dir).await?;
        let modules = Self::new(db);
        let modules = Arc::new(modules);
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
            $pub fn new(db: DbPool) -> Self {
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
