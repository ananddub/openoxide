pub use types::{DatabaseKind, DatabaseOperation, DatabaseOperationResult, DatabaseRecord};

use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

use crate::repository::{
    DatabaseManagementRepository, DeploymentRepository, LibsqlRepository, MariadbRepository,
    MongoRepository, MysqlRepository, PostgresRepository, RedisRepository,
    ResourceDependencyRepository,
};

use crate::core::cache::AppStateCache;

pub struct DatabaseService {
    pub(super) db: Arc<SqlitePool>,
    pub(super) repo_postgres: Arc<PostgresRepository>,
    pub(super) repo_mysql: Arc<MysqlRepository>,
    pub(super) repo_mariadb: Arc<MariadbRepository>,
    pub(super) repo_mongo: Arc<MongoRepository>,
    pub(super) repo_redis: Arc<RedisRepository>,
    pub(super) repo_libsql: Arc<LibsqlRepository>,
    pub(super) repo_deploy: Arc<DeploymentRepository>,
    pub(super) repo_management: Arc<DatabaseManagementRepository>,
    pub(super) repo_dependencies: Arc<ResourceDependencyRepository>,
    pub(super) cache: Arc<AppStateCache>,
}

#[singleton]
impl DatabaseService {
    fn new(
        db: Arc<SqlitePool>,
        repo_postgres: Arc<PostgresRepository>,
        repo_mysql: Arc<MysqlRepository>,
        repo_mariadb: Arc<MariadbRepository>,
        repo_mongo: Arc<MongoRepository>,
        repo_redis: Arc<RedisRepository>,
        repo_libsql: Arc<LibsqlRepository>,
        repo_deploy: Arc<DeploymentRepository>,
        repo_management: Arc<DatabaseManagementRepository>,
        repo_dependencies: Arc<ResourceDependencyRepository>,
        cache: Arc<AppStateCache>,
    ) -> Self {
        Self {
            db,
            repo_postgres,
            repo_mysql,
            repo_mariadb,
            repo_mongo,
            repo_redis,
            repo_libsql,
            repo_deploy,
            repo_management,
            repo_dependencies,
            cache,
        }
    }
}

pub mod crud;
pub mod management;
pub mod operations;
pub mod queries;
pub mod types;
