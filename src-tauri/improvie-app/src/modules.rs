use std::sync::Arc;

use improvie_infra::{modules::RepositoriesModuleImpl, persistence::db::DbPool};

use crate::usecase::health_check::HealthCheckUseCase;

pub struct UsecasesModule {
    health_check_usecase: HealthCheckUseCase<RepositoriesModuleImpl>,
}

impl UsecasesModule {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let db = Arc::new(DbPool::new());
        let repositories = Arc::new(RepositoriesModuleImpl::new(Arc::clone(&db)));
        Self {
            health_check_usecase: HealthCheckUseCase::new(Arc::clone(&repositories)),
        }
    }
}

impl UsecasesModule {
    pub fn health_check_usecase(&self) -> &HealthCheckUseCase<RepositoriesModuleImpl> {
        &self.health_check_usecase
    }
}
