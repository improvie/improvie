use std::sync::Arc;

use crate::{
    macros::def_repositories_module, persistence::db::DbPool,
    repository::health_check::HealthCheckRepositoryImpl,
};

pub struct RepositoriesModuleImpl(Arc<RepositoriesModuleImplInner>);

impl Clone for RepositoriesModuleImpl {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl RepositoriesModuleImpl {
    pub fn new(db: DbPool) -> Self {
        Self(Arc::new(RepositoriesModuleImplInner::new(db)))
    }
}

def_repositories_module!(RepositoriesModuleImpl,
    struct RepositoriesModuleImplInner {
        health_check_repository: HealthCheckRepositoryImpl = HealthCheckRepository,
    }
);
