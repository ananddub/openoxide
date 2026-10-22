//! Shared in-memory SQLite harness for the compute service test suite.
//!
//! Every test gets a fresh database with the full migration set applied plus a
//! small, deterministic seed graph:
//!
//! - organizations 1 (`acme`) and 2 (`other`)
//! - users 1 (`dev@acme.test`), 2 (`dev@other.test`), 3 (`ops@acme.test`)
//! - projects 1 (`acme`) and 2 (`other`)
//! - environments 1 (`production`, acme), 2 (`staging`, acme), 3 (`other`)
//! - servers 1 and 2, both in `acme`
//! - database networks 1/2 on server 1, 3 on server 2, 4 global (no server)
//! - git provider 1, registry 1, ssh key 1

use std::{str::FromStr, sync::Arc};

use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

use crate::{
    core::cache::AppStateCache,
    db::queries::FlashQueries,
    flash_gen::migrations::FlashInitMigrate,
    services::compute::{
        application::{
            ApplicationService, import_export::ApplicationTransferService,
            middleware::ApplicationMiddlewareService, mount::MountService,
            network::ApplicationNetworkService, patch::PatchService, port::PortService,
            redirect::RedirectService, security::SecurityService,
        },
        compose::{
            ComposeService,
            management::{
                ComposeManagementService, ComposeMountService, ComposePatchService,
                ComposeTransferService,
            },
        },
        database::{
            DatabaseService, mount::DatabaseMountService,
            swarm_settings::DatabaseSwarmSettingsService,
        },
        engine::DeploymentService,
        preview::PreviewDeploymentService,
    },
};

pub(crate) const ORG_ID: i64 = 1;
pub(crate) const OTHER_ORG_ID: i64 = 2;
pub(crate) const USER_ID: i64 = 1;
pub(crate) const PROJECT_ID: i64 = 1;
pub(crate) const ENV_ID: i64 = 1;
pub(crate) const STAGING_ENV_ID: i64 = 2;
pub(crate) const OTHER_ENV_ID: i64 = 3;
pub(crate) const SERVER_ID: i64 = 1;
pub(crate) const SERVER_ID_2: i64 = 2;
pub(crate) const OTHER_SERVER_NETWORK_ID: i64 = 3;
pub(crate) const GIT_PROVIDER_ID: i64 = 1;
pub(crate) const REGISTRY_ID: i64 = 1;
pub(crate) const SSH_KEY_ID: i64 = 1;

/// Serializes tests that read or mutate the global `ApplicationState`
/// singleton so parallel test threads cannot observe each other's entries.
pub(crate) async fn state_guard() -> tokio::sync::MutexGuard<'static, ()> {
    static STATE_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());
    STATE_LOCK.lock().await
}

pub(crate) struct TestDb {
    pub pool: Arc<SqlitePool>,
    pub queries: Arc<FlashQueries>,
    pub cache: Arc<AppStateCache>,
}

impl TestDb {
    pub async fn new() -> Self {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .idle_timeout(None)
            .max_lifetime(None)
            .connect_with(
                SqliteConnectOptions::from_str("sqlite::memory:")
                    .unwrap()
                    .foreign_keys(true),
            )
            .await
            .unwrap();
        FlashInitMigrate(&pool).await.unwrap();
        // FlashInitMigrate embeds only a subset of db/migrations; the generated
        // Servers model selects these two columns, so add them to the in-memory
        // schema to keep decoding in sync with the live database.
        for statement in [
            "ALTER TABLE servers ADD COLUMN host_key_fingerprint TEXT NOT NULL DEFAULT ''",
            "ALTER TABLE servers ADD COLUMN host_key_openssh TEXT NOT NULL DEFAULT ''",
        ] {
            sqlx::query(statement).execute(&pool).await.unwrap();
        }
        let pool = Arc::new(pool);
        let queries = Arc::new(FlashQueries::new(pool.clone()));
        let db = Self {
            pool,
            queries,
            cache: Arc::new(AppStateCache::new()),
        };
        db.seed().await;
        db
    }

    async fn seed(&self) {
        let statements = [
            "INSERT INTO organization (id, name, slug, owner_id) VALUES \
             (1, 'Acme', 'acme', 1), (2, 'Other', 'other', 2)",
            "INSERT INTO users (id, email, avatar, password, is_registered, group_id) VALUES \
             (1, 'dev@acme.test', '', 'x', 1, 1), \
             (2, 'dev@other.test', '', 'x', 1, 1), \
             (3, 'ops@acme.test', '', 'x', 1, 1)",
            "INSERT INTO projects (id, name, env_var, organization_id) VALUES \
             (1, 'Platform', '{}', 1), (2, 'Other', '{}', 2)",
            "INSERT INTO environments (id, name, env_var, is_default, project_id) VALUES \
             (1, 'production', '{}', 1, 1), (2, 'staging', '{}', 0, 1), \
             (3, 'other-production', '{}', 1, 2)",
            "INSERT INTO servers (id, name, ip_address, port, username, app_name, \
                 server_status, server_type, enable_docker_cleanup, command, metrics_config, \
                 organization_id) VALUES \
             (1, 'srv-1', '10.0.0.10', 22, 'root', 'srv1', 'ACTIVE', 'DEPLOY', 0, '', '{}', 1), \
             (2, 'srv-2', '10.0.0.11', 22, 'root', 'srv2', 'ACTIVE', 'DEPLOY', 0, '', '{}', 1)",
            "INSERT INTO database_networks (id, name, docker_network_name, external, server_id) \
             VALUES (1, 'backend', 'backend', 0, 1), (2, 'frontend', 'frontend', 0, 1), \
             (3, 'other-net', 'other-net', 0, 2), (4, 'global', 'global', 1, NULL)",
            "INSERT INTO git_providers (id, name, provider_type, config_json, shared, \
                 organization_id) VALUES (1, 'gh', 'GITHUB', '{}', 0, 1)",
            "INSERT INTO registries (id, registry_name, username, password, registry_url, \
                 registry_type, organization_id) VALUES \
             (1, 'ghcr', 'u', 'p', 'ghcr.io', 'GHCR', 1)",
            "INSERT INTO ssh_keys (id, name, private_key, public_key, organization_id) VALUES \
             (1, 'deploy', 'priv', 'pub', 1)",
        ];
        for statement in statements {
            sqlx::query(statement).execute(&*self.pool).await.unwrap();
        }
    }

    pub fn deployments(&self) -> Arc<DeploymentService> {
        Arc::new(DeploymentService::for_tests(
            self.pool.clone(),
            self.queries.clone(),
        ))
    }

    pub fn application(&self) -> ApplicationService {
        ApplicationService {
            db: self.pool.clone(),
            queries: self.queries.clone(),
            deployment_service: self.deployments(),
            cache: self.cache.clone(),
        }
    }

    pub fn compose(&self) -> ComposeService {
        ComposeService {
            queries: self.queries.clone(),
            cache: self.cache.clone(),
        }
    }

    pub fn database(&self) -> DatabaseService {
        DatabaseService {
            queries: self.queries.clone(),
            cache: self.cache.clone(),
        }
    }

    pub fn preview(&self) -> PreviewDeploymentService {
        PreviewDeploymentService::for_tests(self.queries.clone())
    }

    pub fn compose_management(&self) -> ComposeManagementService {
        ComposeManagementService::for_tests(
            Arc::new(self.compose()),
            self.queries.clone(),
            self.deployments(),
            self.cache.clone(),
        )
    }

    pub fn application_network(&self) -> ApplicationNetworkService {
        ApplicationNetworkService::for_tests(self.queries.clone(), self.cache.clone())
    }

    pub fn application_transfer(&self) -> ApplicationTransferService {
        ApplicationTransferService::for_tests(self.queries.clone(), self.cache.clone())
    }

    pub fn port(&self) -> PortService {
        PortService::for_tests(self.queries.clone())
    }

    pub fn mount(&self) -> MountService {
        MountService::for_tests(self.queries.clone())
    }

    pub fn patch(&self) -> PatchService {
        PatchService::for_tests(self.queries.clone())
    }

    pub fn middleware(&self) -> ApplicationMiddlewareService {
        ApplicationMiddlewareService::for_tests(self.queries.clone())
    }

    pub fn redirect(&self) -> RedirectService {
        RedirectService::for_tests(self.queries.clone())
    }

    pub fn security(&self) -> SecurityService {
        SecurityService::for_tests(self.queries.clone())
    }

    pub fn compose_mount(&self) -> ComposeMountService {
        ComposeMountService::for_tests(self.queries.clone())
    }

    pub fn compose_patch(&self) -> ComposePatchService {
        ComposePatchService::for_tests(Arc::new(self.compose()), self.queries.clone())
    }

    pub fn compose_transfer(&self) -> ComposeTransferService {
        ComposeTransferService::for_tests(self.queries.clone(), self.cache.clone())
    }

    pub fn database_mount(&self) -> DatabaseMountService {
        DatabaseMountService::for_tests(Arc::new(self.database()), self.queries.clone())
    }

    pub fn swarm_settings(&self) -> DatabaseSwarmSettingsService {
        DatabaseSwarmSettingsService::new(self.queries.clone())
    }

    pub async fn insert_service(
        &self,
        service_type: &str,
        app_status: &str,
        environment_id: i64,
        server_id: Option<i64>,
    ) -> i64 {
        let app_name = format!(
            "{}-{}",
            service_type.to_ascii_lowercase(),
            uuid::Uuid::new_v4().simple()
        );
        let result = sqlx::query(
            "INSERT INTO services (service_type, name, app_name, app_status, replicas, \
                 network_ids, detach_rustploy_network, environment_id, server_id) \
             VALUES (?, ?, ?, ?, 1, '[]', 0, ?, ?)",
        )
        .bind(service_type)
        .bind(&app_name)
        .bind(&app_name)
        .bind(app_status)
        .bind(environment_id)
        .bind(server_id)
        .execute(&*self.pool)
        .await
        .unwrap();
        result.last_insert_rowid()
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn insert_deployment(
        &self,
        service_id: Option<i64>,
        server_id: Option<i64>,
        status: &str,
        state: &str,
        created_at: i64,
        log_path: &str,
        user_id: i64,
    ) -> i64 {
        let result = sqlx::query(
            "INSERT INTO deployments (title, status, state, log_path, is_preview_deployment, \
                 service_id, server_id, created_at, user_id) \
             VALUES ('Deployment', ?, ?, ?, 0, ?, ?, ?, ?)",
        )
        .bind(status)
        .bind(state)
        .bind(log_path)
        .bind(service_id)
        .bind(server_id)
        .bind(created_at)
        .bind(user_id)
        .execute(&*self.pool)
        .await
        .unwrap();
        result.last_insert_rowid()
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn insert_preview(
        &self,
        base_service_id: i64,
        preview_service_id: Option<i64>,
        status: &str,
        pull_request_number: &str,
        domain: &str,
    ) -> i64 {
        let result = sqlx::query(
            "INSERT INTO preview_deployments (base_service_id, preview_service_id, \
                 provider_type, provider_id, owner, repository, pull_request_number, \
                 source_branch, target_branch, status, domain) \
             VALUES (?, ?, 'GITHUB', 1, 'acme', 'app', ?, 'feature', 'main', ?, ?)",
        )
        .bind(base_service_id)
        .bind(preview_service_id)
        .bind(pull_request_number)
        .bind(status)
        .bind(domain)
        .execute(&*self.pool)
        .await
        .unwrap();
        result.last_insert_rowid()
    }

    pub async fn insert_volume_backup(&self, service_id: i64, enabled: bool) -> i64 {
        let result = sqlx::query(
            "INSERT INTO volume_backups (name, volume_name, prefix, service_type, app_name, \
                 turn_off, cron_expression, enabled, service_id, destination_id, organization_id) \
             VALUES ('backup', 'volume', 'pre', 'APPLICATION', 'app', 0, '0 0 * * *', ?, ?, 1, 1)",
        )
        .bind(i64::from(enabled))
        .bind(service_id)
        .execute(&*self.pool)
        .await
        .unwrap();
        result.last_insert_rowid()
    }
}
