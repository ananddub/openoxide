use std::sync::Arc;

use auto_di::singleton;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::queries::FlashQueries,
    flash_gen::{
        models::{ServerMigrations, Servers},
        server_migrations::{
            ServerMigrationsBeginParams, ServerMigrationsFinishFailureParams,
            ServerMigrationsFinishSuccessParams,
        },
        servers::{
            ServersCreateAndReturnParams, ServersRollbackApplicationServersParams,
            ServersRollbackBuildAssignmentsParams, ServersRollbackCertificatesParams,
            ServersRollbackComposeServersParams, ServersRollbackSchedulesParams,
            ServersUpdateAndReturnParams,
        },
    },
    services::{
        compute::application::{ApplicationOperation, ApplicationService},
        compute::compose::{ComposeOperation, ComposeService},
    },
};

pub struct RemoteServerService {
    queries: Arc<FlashQueries>,
    applications: Arc<ApplicationService>,
    compose: Arc<ComposeService>,
}

/// Resolves the SSH endpoint for a server, preferring an active managed
/// private network address over the public IP.
pub async fn ssh_credentials(
    queries: &FlashQueries,
    server_id: i64,
) -> Result<Option<(String, i64, String, String, String)>, sqlx::Error> {
    let row = match queries.servers_get_ssh_credentials(server_id).await {
        Ok(row) => row,
        Err(sqlx::Error::RowNotFound) => return Ok(None),
        Err(error) => return Err(error),
    };
    let mut selected_ip = row.public_ip.clone();
    if let Some(private_ip) = row.private_ip.as_deref() {
        let mode = row.connection_mode.as_deref().unwrap_or_default();
        let status = row.private_status.as_deref().unwrap_or_default();
        if !private_ip.trim().is_empty()
            && mode.eq_ignore_ascii_case("MANAGED_WIREGUARD")
            && status.eq_ignore_ascii_case("ACTIVE")
        {
            selected_ip = private_ip.trim().to_string();
        }
    }
    let (Some(private_key), Some(public_key)) = (row.private_key, row.public_key) else {
        return Ok(None);
    };
    Ok(Some((selected_ip, row.port, row.username, private_key, public_key)))
}

/// Resolves the raw public SSH endpoint without private network fallback.
pub async fn direct_ssh_credentials(
    queries: &FlashQueries,
    server_id: i64,
) -> Result<Option<(String, i64, String, String, String)>, sqlx::Error> {
    let row = match queries.servers_get_direct_ssh_credentials(server_id).await {
        Ok(row) => row,
        Err(sqlx::Error::RowNotFound) => return Ok(None),
        Err(error) => return Err(error),
    };
    let (Some(private_key), Some(public_key)) = (row.private_key, row.public_key) else {
        return Ok(None);
    };
    Ok(Some((row.ip_address, row.port, row.username, private_key, public_key)))
}

#[singleton]
impl RemoteServerService {
    fn new(
        queries: Arc<FlashQueries>,
        applications: Arc<ApplicationService>,
        compose: Arc<ComposeService>,
    ) -> Self {
        Self {
            queries,
            applications,
            compose,
        }
    }

    pub async fn setup_advertise_addr(
        &self,
        server_id: i64,
        requested: Option<String>,
    ) -> sqlx::Result<Option<String>> {
        if let Some(address) = requested {
            return validate_advertise_addr(address).map(Some);
        }
        let network = match self.queries.server_private_networks_get(server_id).await {
            Ok(network) => network,
            Err(sqlx::Error::RowNotFound) => return Ok(None),
            Err(error) => return Err(error),
        };
        private_advertise_addr(&network.status, network.private_host.as_deref())
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<Servers> {
        self.queries.servers_get_by_id(id).await
    }

    pub async fn list(&self) -> sqlx::Result<Vec<Servers>> {
        self.queries.servers_list_ordered().await
    }

    pub async fn connection_details(
        &self,
        id: i64,
    ) -> sqlx::Result<(Servers, crate::flash_gen::models::SshKeys)> {
        let server = self.get_by_id(id).await?;
        let key_id = server.ssh_key_id.ok_or(sqlx::Error::RowNotFound)?;
        let key = self.queries.ssh_keys_get_by_id(key_id).await?;
        Ok((server, key))
    }

    pub async fn create(&self, input: CreateRemoteServerDto) -> sqlx::Result<Servers> {
        let app_name = generate_app_name(&input.name);
        let server_type = input.server_type.as_str().to_owned();

        self.queries
            .servers_create_and_return(&ServersCreateAndReturnParams {
                name: input.name,
                description: input.description.unwrap_or_default(),
                ip_address: input.ip_address,
                port: input.port,
                username: input.username,
                app_name,
                server_type,
                ssh_key_id: input.ssh_key_id.unwrap_or(0),
                build_memory_limit: input.build_memory_limit.unwrap_or_default(),
                build_cpu_limit: input.build_cpu_limit.unwrap_or_default(),
            })
            .await
    }

    pub async fn patch(&self, id: i64, input: PatchRemoteServerDto) -> sqlx::Result<Servers> {
        let current = self.get_by_id(id).await?;
        self.queries
            .servers_update_and_return(&ServersUpdateAndReturnParams {
                id,
                name: input.name.unwrap_or(current.name),
                description: input
                    .description
                    .or(current.description)
                    .unwrap_or_default(),
                ip_address: input.ip_address.unwrap_or(current.ip_address),
                port: input.port.unwrap_or(current.port),
                username: input.username.unwrap_or(current.username),
                server_status: input
                    .server_status
                    .map(|value| value.as_str().to_owned())
                    .unwrap_or(current.server_status),
                server_type: input
                    .server_type
                    .map(|value| value.as_str().to_owned())
                    .unwrap_or(current.server_type),
                enable_docker_cleanup: input
                    .enable_docker_cleanup
                    .unwrap_or(current.enable_docker_cleanup),
                log_cleanup_cron: input
                    .log_cleanup_cron
                    .or(current.log_cleanup_cron)
                    .unwrap_or_default(),
                command: input.command.unwrap_or(current.command),
                metrics_config: input.metrics_config.unwrap_or(current.metrics_config),
                ssh_key_id: input.ssh_key_id.or(current.ssh_key_id).unwrap_or(0),
                build_memory_limit: input
                    .build_memory_limit
                    .or(current.build_memory_limit)
                    .unwrap_or_default(),
                build_cpu_limit: input
                    .build_cpu_limit
                    .or(current.build_cpu_limit)
                    .unwrap_or_default(),
            })
            .await
    }

    pub async fn set_status(&self, id: i64, status: RemoteServerStatus) -> sqlx::Result<Servers> {
        self.queries
            .servers_set_status(status.as_str(), id)
            .await
    }

    pub async fn touch_test_connection(&self, id: i64) -> sqlx::Result<Servers> {
        let server = self.get_by_id(id).await?;
        let executor =
            crate::services::compute::compose::remote::remote_executor(&self.queries.pool, id)
                .await
                .map_err(|e| {
                    sqlx::Error::Protocol(format!("Could not create SSH executor: {e}"))
                })?;

        let session = executor.connect_session().await.map_err(|e| {
            sqlx::Error::Protocol(format!(
                "SSH connection test failed for {}: {e}",
                server.ip_address
            ))
        })?;

        let (exit_code, _, stderr) = os::ssh::execute_russh_cmd(&session, "echo 'SSH_OK'")
            .await
            .map_err(|e| {
                sqlx::Error::Protocol(format!("SSH test command execution failed: {e}"))
            })?;

        if exit_code != 0 {
            let err_str = String::from_utf8_lossy(&stderr);
            return Err(sqlx::Error::Protocol(format!(
                "SSH test command failed with exit code {exit_code}: {err_str}"
            )));
        }

        if let Some(ssh_key_id) = server.ssh_key_id {
            let _ = self.queries.ssh_keys_touch_ssh_key(ssh_key_id).await;
        }

        // A successful explicit verification is authoritative immediately;
        // do not make the UI wait for the monitoring stream's next heartbeat.
        self.queries
            .servers_set_status(RemoteServerStatus::Active.as_str(), id)
            .await?;

        Ok(server)
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.get_by_id(id).await?;
        let dependencies = self.dependency_counts(id).await?;
        if dependencies.total() > 0 {
            return Err(sqlx::Error::Protocol(format!(
                "server has active dependencies: applications={}, build_assignments={}, compose_projects={}, databases={}, certificates={}, schedules={}",
                dependencies.applications,
                dependencies.build_assignments,
                dependencies.compose_projects,
                dependencies.databases,
                dependencies.certificates,
                dependencies.schedules,
            )));
        }
        if let Ok(private_network_service) =
            auto_di::resolve::<crate::services::infrastructure::server::ServerPrivateNetworkService>().await
        {
            let _ = private_network_service.disable(id).await;
        }
        self.queries.servers_delete(id).await
    }

    async fn dependency_counts(&self, id: i64) -> sqlx::Result<ServerDependencyCounts> {
        Ok(ServerDependencyCounts {
            applications: self.queries.servers_count_applications(id).await?,
            build_assignments: self.queries.servers_count_build_assignments(id).await?,
            compose_projects: self.queries.servers_count_compose_projects(id).await?,
            databases: self.queries.servers_count_databases(id).await?,
            certificates: self.queries.servers_count_certificates(id).await?,
            schedules: self.queries.servers_count_schedules(id).await?,
        })
    }

    pub async fn migrate_dependencies(
        &self,
        source: i64,
        target: i64,
    ) -> sqlx::Result<ServerDependencyMigrationDto> {
        if source == target {
            return Err(sqlx::Error::Protocol(
                "source and target server must differ".into(),
            ));
        }
        self.get_by_id(source).await?;
        self.get_by_id(target).await?;
        let dependencies = self.dependency_counts(source).await?;
        if dependencies.databases > 0 {
            return Err(sqlx::Error::Protocol(format!(
                "stateful migration is required for {} databases; database metadata was not moved",
                dependencies.databases
            )));
        }
        let application_ids = self
            .queries
            .servers_migratable_application_ids(source)
            .await?;
        let build_ids = self
            .queries
            .servers_migratable_build_service_ids(source)
            .await?;
        let compose_ids = self.queries.servers_migratable_compose_ids(source).await?;
        let certificate_ids = self
            .queries
            .servers_migratable_certificate_ids(source)
            .await?;
        let schedule_ids = self
            .queries
            .servers_migratable_schedule_ids(source)
            .await?;
        let migration_id = Uuid::new_v4().simple().to_string();
        let encode = |values: &[i64]| {
            serde_json::to_string(values).map_err(|error| sqlx::Error::Protocol(error.to_string()))
        };
        let mut service_ids = application_ids.clone();
        service_ids.extend_from_slice(&compose_ids);
        self.queries
            .server_migrations_begin(&ServerMigrationsBeginParams {
                id: migration_id.clone(),
                source_server_id: source,
                target_server_id: target,
                service_ids: encode(&service_ids)?,
                build_service_ids: encode(&build_ids)?,
                certificate_ids: encode(&certificate_ids)?,
                schedule_ids: encode(&schedule_ids)?,
            })
            .await?;
        let mut stop_failures = Vec::new();
        for id in &application_ids {
            if let Err(error) = self.applications.cancel_operation(*id).await {
                stop_failures.push(format!("application {id}: {error}"));
            }
        }
        for id in &compose_ids {
            if let Err(error) = self.compose.cancel_operation(*id).await {
                stop_failures.push(format!("compose {id}: {error}"));
            }
        }
        if !stop_failures.is_empty() {
            let error = format!(
                "source workloads could not be stopped: {}",
                stop_failures.join("; ")
            );
            self.queries
                .server_migrations_finish_failure(&ServerMigrationsFinishFailureParams {
                    queued_services: 0,
                    queued_total: 0,
                    error,
                    id: migration_id.clone(),
                })
                .await?;
            return self.migration_status(&migration_id).await;
        }
        self.migrate_server_assignments(source, target).await?;
        let mut queued_applications = 0_i64;
        let mut queued_compose = 0_i64;
        let mut failures = Vec::new();
        for id in &application_ids {
            match self
                .applications
                .run_operation(*id, ApplicationOperation::Redeploy)
                .await
            {
                Ok(_) => queued_applications += 1,
                Err(error) => failures.push(format!("application {id}: {error}")),
            }
        }
        for id in &compose_ids {
            match self
                .compose
                .run_operation(*id, ComposeOperation::Redeploy)
                .await
            {
                Ok(_) => queued_compose += 1,
                Err(error) => failures.push(format!("compose {id}: {error}")),
            }
        }
        if let Some(error) = (!failures.is_empty()).then(|| failures.join("; ")) {
            self.queries
                .server_migrations_finish_failure(&ServerMigrationsFinishFailureParams {
                    queued_services: queued_applications,
                    queued_total: queued_compose,
                    error,
                    id: migration_id.clone(),
                })
                .await?;
        } else {
            self.queries
                .server_migrations_finish_success(&ServerMigrationsFinishSuccessParams {
                    queued_services: queued_applications,
                    queued_total: queued_compose,
                    id: migration_id.clone(),
                })
                .await?;
        }
        let migration = self.queries.server_migrations_get(&migration_id).await?;
        self.migration_dto(migration, dependencies.databases).await
    }

    async fn migrate_server_assignments(&self, source: i64, target: i64) -> sqlx::Result<()> {
        self.queries
            .servers_migrate_application_servers(target, source)
            .await?;
        self.queries
            .servers_migrate_build_assignments(target, source)
            .await?;
        self.queries
            .servers_migrate_compose_servers(target, source)
            .await?;
        self.queries
            .servers_migrate_certificates(target, source)
            .await?;
        self.queries
            .servers_migrate_schedules(target, source)
            .await?;
        Ok(())
    }

    pub async fn migration_status(
        &self,
        id: &str,
    ) -> sqlx::Result<ServerDependencyMigrationDto> {
        let migration = self.queries.server_migrations_get(id).await?;
        self.migration_dto(migration, 0).await
    }

    async fn migration_dto(
        &self,
        migration: ServerMigrations,
        databases: i64,
    ) -> sqlx::Result<ServerDependencyMigrationDto> {
        let applications = self
            .queries
            .servers_list_migrated_applications(&migration.service_ids)
            .await?;
        let compose = self
            .queries
            .servers_list_migrated_compose(&migration.service_ids)
            .await?;
        let mut dto = ServerDependencyMigrationDto::try_from(migration)?;
        dto.applications = applications.len() as i64;
        dto.compose_projects = compose.len() as i64;
        dto.databases = databases;
        Ok(dto)
    }

    pub async fn rollback_migration(
        &self,
        id: &str,
    ) -> sqlx::Result<ServerDependencyMigrationDto> {
        let migration = self.queries.server_migrations_get(id).await?;
        if migration.status == "ROLLED_BACK" {
            return self.migration_dto(migration, 0).await;
        }
        let decode = |value: &str| {
            serde_json::from_str::<Vec<i64>>(value)
                .map_err(|error| sqlx::Error::Protocol(error.to_string()))
        };
        let application_ids = self
            .queries
            .servers_list_migrated_applications(&migration.service_ids)
            .await?;
        let compose_ids = self
            .queries
            .servers_list_migrated_compose(&migration.service_ids)
            .await?;
        let build_ids = decode(&migration.build_service_ids)?;
        let certificate_ids = decode(&migration.certificate_ids)?;
        let schedule_ids = decode(&migration.schedule_ids)?;
        self.rollback_server_assignments(
            migration.source_server_id,
            migration.target_server_id,
            &application_ids,
            &build_ids,
            &compose_ids,
            &certificate_ids,
            &schedule_ids,
        )
        .await?;
        self.queries.server_migrations_mark_rolled_back(id).await?;
        let mut failures = Vec::new();
        for resource_id in application_ids {
            if let Err(error) = self
                .applications
                .run_operation(resource_id, ApplicationOperation::Redeploy)
                .await
            {
                failures.push(format!("application {resource_id}: {error}"));
            }
        }
        for resource_id in compose_ids {
            if let Err(error) = self
                .compose
                .run_operation(resource_id, ComposeOperation::Redeploy)
                .await
            {
                failures.push(format!("compose {resource_id}: {error}"));
            }
        }
        if !failures.is_empty() {
            return Err(sqlx::Error::Protocol(format!(
                "metadata rolled back but some source redeploys failed: {}",
                failures.join("; ")
            )));
        }
        self.migration_status(id).await
    }

    async fn rollback_server_assignments(
        &self,
        source: i64,
        target: i64,
        application_ids: &[i64],
        build_ids: &[i64],
        compose_ids: &[i64],
        certificate_ids: &[i64],
        schedule_ids: &[i64],
    ) -> sqlx::Result<()> {
        for id in application_ids {
            self.queries
                .servers_rollback_application_servers(&ServersRollbackApplicationServersParams {
                    server_id: source,
                    id: *id,
                    server_id2: target,
                })
                .await?;
        }
        for id in compose_ids {
            self.queries
                .servers_rollback_compose_servers(&ServersRollbackComposeServersParams {
                    server_id: source,
                    id: *id,
                    server_id2: target,
                })
                .await?;
        }
        for id in build_ids {
            self.queries
                .servers_rollback_build_assignments(&ServersRollbackBuildAssignmentsParams {
                    build_server_id: source,
                    service_id: *id,
                    build_server_id2: target,
                })
                .await?;
        }
        for id in certificate_ids {
            self.queries
                .servers_rollback_certificates(&ServersRollbackCertificatesParams {
                    server_id: source,
                    id: *id,
                    server_id2: target,
                })
                .await?;
        }
        for id in schedule_ids {
            self.queries
                .servers_rollback_schedules(&ServersRollbackSchedulesParams {
                    server_id: source,
                    id: *id,
                    server_id2: target,
                })
                .await?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoteServerStatus {
    Active,
    Inactive,
}

impl RemoteServerStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "ACTIVE",
            Self::Inactive => "INACTIVE",
        }
    }
}

fn private_advertise_addr(status: &str, private_host: Option<&str>) -> sqlx::Result<Option<String>> {
    if !status.eq_ignore_ascii_case("ACTIVE") {
        return Ok(None);
    }
    private_host
        .map(|host| validate_advertise_addr(host.to_owned()))
        .transpose()
}

fn validate_advertise_addr(value: String) -> sqlx::Result<String> {
    let address = value.parse::<std::net::IpAddr>().map_err(|error| {
        sqlx::Error::Protocol(format!(
            "invalid Swarm advertise address {value:?}: {error}"
        ))
    })?;
    if address.is_unspecified() || address.is_loopback() || address.is_multicast() {
        return Err(sqlx::Error::Protocol(
            "Swarm advertise address must be a routable unicast IP".into(),
        ));
    }
    Ok(address.to_string())
}

fn generate_app_name(name: &str) -> String {
    let mut slug = String::new();
    let mut previous_dash = false;

    for ch in name.to_lowercase().chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch);
            previous_dash = false;
        } else if !previous_dash && !slug.is_empty() {
            slug.push('-');
            previous_dash = true;
        }
    }

    let slug = slug.trim_matches('-');
    let base = if slug.is_empty() { "server" } else { slug };
    let suffix = Uuid::new_v4().simple().to_string();
    format!("{}-{}", base, &suffix[..6])
}

// ── JSON payload contract (moved from the retired api::dto layer) ──────

#[derive(Debug, Validate, Deserialize)]
pub struct CreateRemoteServerDto {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(max = 1_000))]
    pub description: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub ip_address: String,
    #[serde(default = "default_port")]
    pub port: i64,
    #[serde(default = "default_username")]
    pub username: String,
    #[serde(default)]
    pub server_type: RemoteServerTypeDto,
    pub ssh_key_id: Option<i64>,
    pub build_memory_limit: Option<String>,
    pub build_cpu_limit: Option<String>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct PatchRemoteServerDto {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    #[validate(length(max = 1_000))]
    pub description: Option<String>,
    pub ip_address: Option<String>,
    pub port: Option<i64>,
    pub username: Option<String>,
    pub server_status: Option<RemoteServerStatusDto>,
    pub server_type: Option<RemoteServerTypeDto>,
    pub enable_docker_cleanup: Option<i64>,
    pub log_cleanup_cron: Option<String>,
    pub command: Option<String>,
    pub metrics_config: Option<String>,
    pub ssh_key_id: Option<i64>,
    pub build_memory_limit: Option<String>,
    pub build_cpu_limit: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RemoteServerResponseDto {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub ip_address: String,
    pub port: i64,
    pub username: String,
    pub app_name: String,
    pub server_status: String,
    pub server_type: String,
    pub enable_docker_cleanup: i64,
    pub log_cleanup_cron: Option<String>,
    pub command: String,
    pub metrics_config: String,
    pub ssh_key_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
    pub build_memory_limit: Option<String>,
    pub build_cpu_limit: Option<String>,
}

impl From<Servers> for RemoteServerResponseDto {
    fn from(value: Servers) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            ip_address: value.ip_address,
            port: value.port,
            username: value.username,
            app_name: value.app_name,
            server_status: value.server_status,
            server_type: value.server_type,
            enable_docker_cleanup: value.enable_docker_cleanup,
            log_cleanup_cron: value.log_cleanup_cron,
            command: value.command,
            metrics_config: value.metrics_config,
            ssh_key_id: value.ssh_key_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
            build_memory_limit: value.build_memory_limit,
            build_cpu_limit: value.build_cpu_limit,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RemoteServerActionResponseDto {
    pub server: RemoteServerResponseDto,
    pub action: RemoteServerAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RemoteServerAction {
    Activate,
    Deactivate,
    TestConnection,
}

#[derive(Debug, Validate, Deserialize)]
pub struct MigrateServerDependenciesDto {
    #[validate(range(min = 1))]
    pub target_server_id: i64,
}

#[derive(Debug, Clone, Copy)]
pub struct ServerDependencyCounts {
    pub applications: i64,
    pub build_assignments: i64,
    pub compose_projects: i64,
    pub databases: i64,
    pub certificates: i64,
    pub schedules: i64,
}

impl ServerDependencyCounts {
    pub fn total(&self) -> i64 {
        self.applications
            + self.build_assignments
            + self.compose_projects
            + self.databases
            + self.certificates
            + self.schedules
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ServerDependencyMigrationDto {
    pub migration_id: String,
    pub status: String,
    pub source_server_id: i64,
    pub target_server_id: i64,
    pub applications: i64,
    pub build_assignments: i64,
    pub compose_projects: i64,
    pub databases: i64,
    pub certificates: i64,
    pub schedules: i64,
    pub queued_applications: i64,
    pub queued_compose_projects: i64,
    pub error: Option<String>,
}

impl TryFrom<ServerMigrations> for ServerDependencyMigrationDto {
    type Error = sqlx::Error;

    fn try_from(value: ServerMigrations) -> Result<Self, Self::Error> {
        let build: Vec<i64> = serde_json::from_str(&value.build_service_ids)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let certificates: Vec<i64> = serde_json::from_str(&value.certificate_ids)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let schedules: Vec<i64> = serde_json::from_str(&value.schedule_ids)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        Ok(Self {
            migration_id: value.id,
            status: value.status,
            source_server_id: value.source_server_id,
            target_server_id: value.target_server_id,
            // Per-type service splits are filled in by the service via
            // servers_list_migrated_{applications,compose}.
            applications: 0,
            build_assignments: build.len() as i64,
            compose_projects: 0,
            databases: 0,
            certificates: certificates.len() as i64,
            schedules: schedules.len() as i64,
            queued_applications: value.queued_services,
            queued_compose_projects: value.queued_total,
            error: value.error,
        })
    }
}

fn default_port() -> i64 {
    22
}

fn default_username() -> String {
    "root".into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RemoteServerTypeDto {
    Deploy,
    Build,
}

impl Default for RemoteServerTypeDto {
    fn default() -> Self {
        Self::Deploy
    }
}

impl RemoteServerTypeDto {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Deploy => "DEPLOY",
            Self::Build => "BUILD",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RemoteServerStatusDto {
    Active,
    Inactive,
}

impl RemoteServerStatusDto {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "ACTIVE",
            Self::Inactive => "INACTIVE",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swarm_advertise_address_accepts_private_provider_ips() {
        assert_eq!(
            validate_advertise_addr("100.64.10.2".into()).unwrap(),
            "100.64.10.2"
        );
        assert_eq!(
            validate_advertise_addr("10.77.8.2".into()).unwrap(),
            "10.77.8.2"
        );
    }

    #[test]
    fn swarm_advertise_address_rejects_unroutable_values() {
        assert!(validate_advertise_addr("127.0.0.1".into()).is_err());
        assert!(validate_advertise_addr("0.0.0.0".into()).is_err());
        assert!(validate_advertise_addr("tailscale0".into()).is_err());
    }

    #[test]
    fn totals_every_blocking_dependency() {
        let counts = ServerDependencyCounts {
            applications: 1,
            build_assignments: 1,
            compose_projects: 1,
            databases: 1,
            certificates: 1,
            schedules: 1,
        };
        assert_eq!(counts.total(), 6);
    }

    #[test]
    fn private_advertise_addr_requires_active_status() {
        assert!(private_advertise_addr("INACTIVE", Some("10.0.0.5"))
            .unwrap()
            .is_none());
        assert_eq!(
            private_advertise_addr("ACTIVE", Some("10.0.0.5")).unwrap(),
            Some("10.0.0.5".to_string())
        );
    }
}
