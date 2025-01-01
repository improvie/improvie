use std::{path::PathBuf, sync::Arc};

use improvie_infra::{modules::RepositoriesModuleImpl, persistence::db::DbPool};
use improvie_logic::AppResult;

use crate::usecase::health_check::HealthCheckUseCase;

pub struct UsecasesModule {
    health_check_usecase: HealthCheckUseCase<RepositoriesModuleImpl>,
}

impl UsecasesModule {
    pub async fn new(data_dir: PathBuf) -> AppResult<Self> {
        let db = Arc::new(DbPool::new(data_dir).await?);
        let repositories = Arc::new(RepositoriesModuleImpl::new(Arc::clone(&db)));
        Ok(Self {
            health_check_usecase: HealthCheckUseCase::new(Arc::clone(&repositories)),
        })
    }
}

impl UsecasesModule {
    pub fn health_check_usecase(&self) -> &HealthCheckUseCase<RepositoriesModuleImpl> {
        &self.health_check_usecase
    }
}
