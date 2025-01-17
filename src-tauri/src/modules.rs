use improvie_app::usecase::halth_check::HealthCheckUseCase;
use improvie_infra::{modules::RepositoriesModuleImpl, persistence::db::DbPool};

use crate::macros::def_modules;

def_modules!(
    RepositoriesModuleImpl,
    pub struct Modules {
        health_check_use_case: HealthCheckUseCase,
    }
);
