use std::sync::Arc;

use improvie_domain::modules::RepositoriesModule;

use crate::{persistence::db::DbPool, repository::health_check::HealthCheckRepositoryImpl};

pub struct RepositoriesModuleImpl {
    health_check_repository: Arc<HealthCheckRepositoryImpl>,
}

impl RepositoriesModule for RepositoriesModuleImpl {
    type HealthCheckRepository = HealthCheckRepositoryImpl;

    fn health_check_repository(&self) -> &Self::HealthCheckRepository {
        &self.health_check_repository
    }
}

impl RepositoriesModuleImpl {
    pub fn new(db: DbPool) -> Self {
        Self {
            health_check_repository: Arc::new(HealthCheckRepositoryImpl::new(db.clone())),
        }
    }
}
