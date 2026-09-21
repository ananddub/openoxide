use std::collections::HashSet;
use std::sync::Arc;
use std::sync::Mutex;

use auto_di::singleton;
use serde::{Deserialize, Serialize};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::queries::FlashQueries,
    flash_gen::{
        application_migrations::ApplicationMigrationsBeginParams,
        models::{ApplicationMigrations, MigrationObjects, ServerMigrations, Servers},
        server_migrations::{
            ServerMigrationsBeginParams, ServerMigrationsFinishFailureParams,
            ServerMigrationsFinishSuccessParams,
        },
        servers::{
            ServersCreateAndReturnParams, ServersMigrateApplicationServersByIdsParams,
            ServersMigrateBuildAssignmentsByIdsParams, ServersMigrateCertificatesByIdsParams,
            ServersMigrateComposeServersByIdsParams, ServersMigrateSchedulesByIdsParams,
            ServersRollbackApplicationServersParams, ServersRollbackBuildAssignmentsParams,
            ServersRollbackCertificatesParams, ServersRollbackComposeServersParams,
            ServersRollbackSchedulesParams, ServersUpdateAndReturnParams,
        },
    },
    services::{
        compute::application::{ApplicationOperation, ApplicationService},
        compute::compose::{ComposeOperation, ComposeService},
        migration::{
            CancelSignal, MigratedServiceKind, MigrationOrchestrator, MigrationScope,
            StatefulMigrationRequest,
        },
    },
    utils::paths::OpenOxidePaths,
};

fn now_unix() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or_default()
}

pub struct RemoteServerService {
    queries: Arc<FlashQueries>,
    applications: Arc<ApplicationService>,
    compose: Arc<ComposeService>,
    /// Migration ids with an orchestrator task alive in *this* process —
    /// the guard that makes a re-trigger during a live run a no-op resume
    /// instead of a second concurrent driver.
    active_migrations: Arc<Mutex<HashSet<String>>>,
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
            active_migrations: Arc::new(Mutex::new(HashSet::new())),
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

    /// Reject access to a server that is not owned by the caller's active
    /// organization. `get_by_id` stays organization-agnostic because the
    /// deployment engine, monitoring reconciler and SSH plumbing resolve
    /// servers by id with no request context; only the user-facing
    /// controller paths call through this guard.
    pub async fn get_in_org(&self, id: i64, organization_id: i64) -> sqlx::Result<Servers> {
        let server = self.get_by_id(id).await?;
        if server.organization_id != organization_id {
            return Err(sqlx::Error::RowNotFound);
        }
        Ok(server)
    }

    pub async fn list(&self, organization_id: i64) -> sqlx::Result<Vec<Servers>> {
        self.queries.servers_list_ordered(organization_id).await
    }

    /// Every server across all organizations. Only for background engine
    /// loops (monitoring reconcile, alert evaluation) that have no request
    /// organization; user-facing paths must use `list(organization_id)`.
    pub async fn list_all_servers(&self) -> sqlx::Result<Vec<Servers>> {
        self.queries.servers_get_all().await
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

    pub async fn create(
        &self,
        input: CreateRemoteServerDto,
        organization_id: i64,
    ) -> sqlx::Result<Servers> {
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
                organization_id,
            })
            .await
    }

    pub async fn patch(
        &self,
        id: i64,
        input: PatchRemoteServerDto,
        organization_id: i64,
    ) -> sqlx::Result<Servers> {
        let current = self.get_in_org(id, organization_id).await?;
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
                organization_id,
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

    pub async fn delete(&self, id: i64, organization_id: i64) -> sqlx::Result<()> {
        self.get_in_org(id, organization_id).await?;
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
        self.queries.servers_delete(id, organization_id).await
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
        let scope = MigrationScope {
            application_ids: self
                .queries
                .servers_migratable_application_ids(source)
                .await?,
            compose_ids: self.queries.servers_migratable_compose_ids(source).await?,
            build_ids: self
                .queries
                .servers_migratable_build_service_ids(source)
                .await?,
            certificate_ids: self
                .queries
                .servers_migratable_certificate_ids(source)
                .await?,
            schedule_ids: self
                .queries
                .servers_migratable_schedule_ids(source)
                .await?,
        };
        let migration_id = Uuid::new_v4().simple().to_string();
        self.begin_migration(&migration_id, source, target, &scope)
            .await?;
        if !self
            .run_migration_tail(&migration_id, source, target, &scope, true)
            .await?
        {
            return self.migration_status(&migration_id).await;
        }
        let migration = self.queries.server_migrations_get(&migration_id).await?;
        self.migration_dto(migration, dependencies.databases).await
    }

    /// Per-service (selective) metadata migration over an explicit id scope.
    /// Repoints only the listed ids (validating each still belongs to `source`)
    /// and redeploys them on the target. Lays the groundwork for the stateful
    /// runner (S4+); the whole-server path above feeds every migratable id.
    pub async fn migrate_scoped(
        &self,
        source: i64,
        target: i64,
        scope: &MigrationScope,
    ) -> sqlx::Result<ServerDependencyMigrationDto> {
        if source == target {
            return Err(sqlx::Error::Protocol(
                "source and target server must differ".into(),
            ));
        }
        let migration_id = Uuid::new_v4().simple().to_string();
        self.begin_migration(&migration_id, source, target, scope)
            .await?;
        if !self
            .run_migration_tail(&migration_id, source, target, scope, false)
            .await?
        {
            return self.migration_status(&migration_id).await;
        }
        let migration = self.queries.server_migrations_get(&migration_id).await?;
        self.migration_dto(migration, 0).await
    }

    async fn begin_migration(
        &self,
        migration_id: &str,
        source: i64,
        target: i64,
        scope: &MigrationScope,
    ) -> sqlx::Result<()> {
        let encode = |values: &[i64]| {
            serde_json::to_string(values).map_err(|error| sqlx::Error::Protocol(error.to_string()))
        };
        self.queries
            .server_migrations_begin(&ServerMigrationsBeginParams {
                id: migration_id.to_string(),
                source_server_id: source,
                target_server_id: target,
                service_ids: encode(&scope.service_ids())?,
                build_service_ids: encode(&scope.build_ids)?,
                certificate_ids: encode(&scope.certificate_ids)?,
                schedule_ids: encode(&scope.schedule_ids)?,
            })
            .await?;
        Ok(())
    }

    /// Shared stop → repoint → redeploy → record tail. Returns `Ok(false)` when
    /// a source workload could not be stopped (row already marked FAILED);
    /// `Ok(true)` once the redeploy outcome has been recorded.
    async fn run_migration_tail(
        &self,
        migration_id: &str,
        source: i64,
        target: i64,
        scope: &MigrationScope,
        whole_server: bool,
    ) -> sqlx::Result<bool> {
        let mut stop_failures = Vec::new();
        for id in &scope.application_ids {
            if let Err(error) = self.applications.cancel_operation(*id).await {
                stop_failures.push(format!("application {id}: {error}"));
            }
        }
        for id in &scope.compose_ids {
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
                    id: migration_id.to_string(),
                })
                .await?;
            return Ok(false);
        }
        if whole_server {
            self.migrate_server_assignments(source, target).await?;
        } else {
            self.migrate_scoped_assignments(source, target, scope).await?;
        }
        let mut queued_applications = 0_i64;
        let mut queued_compose = 0_i64;
        let mut failures = Vec::new();
        for id in &scope.application_ids {
            match self
                .applications
                .run_operation(*id, ApplicationOperation::Redeploy, 0)
                .await
            {
                Ok(_) => queued_applications += 1,
                Err(error) => failures.push(format!("application {id}: {error}")),
            }
        }
        for id in &scope.compose_ids {
            match self
                .compose
                .run_operation(*id, ComposeOperation::Redeploy, 0)
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
                    id: migration_id.to_string(),
                })
                .await?;
        } else {
            self.queries
                .server_migrations_finish_success(&ServerMigrationsFinishSuccessParams {
                    queued_services: queued_applications,
                    queued_total: queued_compose,
                    id: migration_id.to_string(),
                })
                .await?;
        }
        Ok(true)
    }

    /// Flip only the scoped ids to `target`, each validated against `source`
    /// in-statement. Empty lists are skipped.
    async fn migrate_scoped_assignments(
        &self,
        source: i64,
        target: i64,
        scope: &MigrationScope,
    ) -> sqlx::Result<()> {
        let encode = |values: &[i64]| -> sqlx::Result<String> {
            serde_json::to_string(values).map_err(|error| sqlx::Error::Protocol(error.to_string()))
        };
        if !scope.application_ids.is_empty() {
            self.queries
                .servers_migrate_application_servers_by_ids(
                    &ServersMigrateApplicationServersByIdsParams {
                        server_id: target,
                        server_id2: source,
                        id_list: encode(&scope.application_ids)?,
                    },
                )
                .await?;
        }
        if !scope.build_ids.is_empty() {
            self.queries
                .servers_migrate_build_assignments_by_ids(
                    &ServersMigrateBuildAssignmentsByIdsParams {
                        build_server_id: target,
                        build_server_id2: source,
                        service_id_list: encode(&scope.build_ids)?,
                    },
                )
                .await?;
        }
        if !scope.compose_ids.is_empty() {
            self.queries
                .servers_migrate_compose_servers_by_ids(
                    &ServersMigrateComposeServersByIdsParams {
                        server_id: target,
                        server_id2: source,
                        id_list: encode(&scope.compose_ids)?,
                    },
                )
                .await?;
        }
        if !scope.certificate_ids.is_empty() {
            self.queries
                .servers_migrate_certificates_by_ids(&ServersMigrateCertificatesByIdsParams {
                    server_id: target,
                    server_id2: source,
                    id_list: encode(&scope.certificate_ids)?,
                })
                .await?;
        }
        if !scope.schedule_ids.is_empty() {
            self.queries
                .servers_migrate_schedules_by_ids(&ServersMigrateSchedulesByIdsParams {
                    server_id: target,
                    server_id2: source,
                    id_list: encode(&scope.schedule_ids)?,
                })
                .await?;
        }
        Ok(())
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
                .run_operation(resource_id, ApplicationOperation::Redeploy, 0)
                .await
            {
                failures.push(format!("application {resource_id}: {error}"));
            }
        }
        for resource_id in compose_ids {
            if let Err(error) = self
                .compose
                .run_operation(resource_id, ComposeOperation::Redeploy, 0)
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

    /// S7 cooperative cancellation for a STATEFUL (`application_migrations`)
    /// migration: verifies org ownership, refuses anything already terminal,
    /// and flips `cancel_requested` — the future runner polls it at every
    /// phase/object boundary (see `migration::quiesce::CancelSignal`).
    pub async fn request_stateful_migration_cancel(
        &self,
        id: &str,
        organization_id: i64,
    ) -> sqlx::Result<crate::flash_gen::models::ApplicationMigrations> {
        let row = self.queries.application_migrations_get(id).await?;
        if row.organization_id != organization_id {
            // Do not disclose existence across tenants.
            return Err(sqlx::Error::RowNotFound);
        }
        if row.status != "RUNNING" {
            return Err(sqlx::Error::Protocol(format!(
                "migration {id} is {} and can no longer be cancelled",
                row.status
            )));
        }
        self.queries.application_migrations_request_cancel(id).await?;
        self.queries.application_migrations_get(id).await
    }

    /// Trigger (or resume) a stateful migration of ONE application/compose
    /// service: the source workload stays authoritative until the runner
    /// reaches REPOINT, byte checkpoints make a re-run resumable, and a
    /// live in-process run turns a re-trigger into a plain status return.
    pub async fn migrate_service(
        &self,
        service_id: i64,
        target: i64,
        organization_id: i64,
    ) -> sqlx::Result<ApplicationMigrations> {
        let service = self.queries.get_service_by_i_d(service_id).await?;
        if self
            .queries
            .services_belongs_to_organization(service_id, organization_id)
            .await?
            == 0
        {
            // Do not disclose existence across tenants.
            return Err(sqlx::Error::RowNotFound);
        }
        let kind = match service.service_type.as_str() {
            "APPLICATION" => MigratedServiceKind::Application,
            "COMPOSE" => MigratedServiceKind::Compose,
            other => {
                return Err(sqlx::Error::Protocol(format!(
                    "{other} services cannot be statefully migrated yet — supported: \
                     APPLICATION, COMPOSE"
                )));
            }
        };
        let source = service.server_id.ok_or_else(|| {
            sqlx::Error::Protocol("service is not assigned to a server".into())
        })?;
        if source == target {
            return Err(sqlx::Error::Protocol(
                "source and target server must differ".into(),
            ));
        }
        self.get_in_org(source, organization_id).await?;
        self.get_in_org(target, organization_id).await?;

        // Resume rule: a RUNNING row keeps its id (checkpoints are keyed by
        // (migration_id, obj_key)); a FAILED/SUCCEEDED history never blocks
        // a fresh attempt.
        let history = self
            .queries
            .application_migrations_list_by_service(service_id)
            .await?;
        if let Some(other) = history
            .iter()
            .find(|row| row.status == "RUNNING" && row.target_server_id != target)
        {
            return Err(sqlx::Error::Protocol(format!(
                "migration {} is already running for this service toward server {}",
                other.id, other.target_server_id
            )));
        }
        let running = history.iter().find(|row| row.status == "RUNNING");
        let (row, fresh) = match running {
            Some(row) => (row.clone(), false),
            None => {
                let now = now_unix();
                let migration_id = Uuid::new_v4().simple().to_string();
                let log_path = OpenOxidePaths::from_env().migration_log_file(&migration_id);
                let row = self
                    .queries
                    .application_migrations_begin(&ApplicationMigrationsBeginParams {
                        id: migration_id,
                        organization_id,
                        service_id,
                        source_server_id: source,
                        target_server_id: target,
                        log_path,
                        created_at: now,
                        updated_at: now,
                    })
                    .await?;
                (row, true)
            }
        };
        // `insert` reports whether the id was previously absent; absent +
        // non-fresh means the driver died with an old panel process, so the
        // re-trigger legitimately re-drives (resumes) the same row.
        let already_driving = self
            .active_migrations
            .lock()
            .map(|mut active| {
                let inserted = active.insert(row.id.clone());
                fresh || inserted
            })
            .unwrap_or(true);
        if already_driving {
            self.spawn_orchestrator(
                row.clone(),
                kind,
                service.app_name.clone(),
                source,
                target,
                organization_id,
            );
        }
        Ok(row)
    }

    fn spawn_orchestrator(
        &self,
        row: ApplicationMigrations,
        kind: MigratedServiceKind,
        app_name: String,
        source: i64,
        target: i64,
        organization_id: i64,
    ) {
        let orchestrator = MigrationOrchestrator::new(
            Arc::clone(&self.queries),
            Arc::clone(&self.applications),
            Arc::clone(&self.compose),
        );
        let active = Arc::clone(&self.active_migrations);
        let req = StatefulMigrationRequest {
            migration_id: row.id.clone(),
            service_id: row.service_id,
            kind,
            app_name,
            source_server_id: source,
            target_server_id: target,
            organization_id,
        };
        let migration_id = row.id.clone();
        tokio::spawn(async move {
            // Cancellation arrives through the DB flag flipped by the
            // CancelMigration RPC; the token stays a shutdown hook.
            let cancel = CancelSignal::new(CancellationToken::new());
            orchestrator.run(req, cancel).await;
            if let Ok(mut active) = active.lock() {
                active.remove(&migration_id);
            }
        });
    }

    /// Migrations recorded for one service, newest first, tenant-scoped.
    pub async fn list_service_migrations(
        &self,
        service_id: i64,
        organization_id: i64,
    ) -> sqlx::Result<Vec<ApplicationMigrations>> {
        let rows = self
            .queries
            .application_migrations_list_by_service(service_id)
            .await?;
        Ok(rows
            .into_iter()
            .filter(|row| row.organization_id == organization_id)
            .collect())
    }

    /// Per-object checkpoints of one migration (tenant-scoped via the row).
    pub async fn list_migration_objects(
        &self,
        id: &str,
        organization_id: i64,
    ) -> sqlx::Result<Vec<MigrationObjects>> {
        let row = self.queries.application_migrations_get(id).await?;
        if row.organization_id != organization_id {
            return Err(sqlx::Error::RowNotFound);
        }
        self.queries.migration_objects_list_by_migration(id).await
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
