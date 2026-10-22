use auto_di::resolve;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::db::queries::FlashQueries;
use crate::flash_gen::resource_dependencies::ServiceDependencyCountsParams;
use crate::utils::exec::{RemoteExecutor, SshAuth, SshHostKey};

pub(super) fn generate_app_name(name: &str) -> String {
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
    let base = if slug.is_empty() { "compose" } else { slug };
    let suffix = Uuid::new_v4().simple().to_string();
    format!("{}-{}", base, &suffix[..6])
}

pub(super) async fn update_git_watch_paths(
    queries: &FlashQueries,
    service_id: i64,
    watch_paths: Option<&str>,
) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE service_git SET watch_paths = COALESCE(?, watch_paths) WHERE service_id = ?",
    )
    .bind(watch_paths)
    .bind(service_id)
    .execute(&queries.pool)
    .await?;
    Ok(())
}

pub(super) async fn update_git_enable_submodules(
    queries: &FlashQueries,
    service_id: i64,
    enable_submodules: Option<i64>,
) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE service_git SET enable_submodules = COALESCE(?, enable_submodules) WHERE service_id = ?",
    )
    .bind(enable_submodules)
    .bind(service_id)
    .execute(&queries.pool)
    .await?;
    Ok(())
}

/// No generated query covers `services.name`/`services.description` updates.
pub(super) async fn update_service_identity(
    queries: &FlashQueries,
    id: i64,
    name: Option<&str>,
    description: Option<&str>,
) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE services SET name = COALESCE(?, name), description = COALESCE(?, description) WHERE id = ?",
    )
    .bind(name)
    .bind(description)
    .bind(id)
    .execute(&queries.pool)
    .await?;
    Ok(())
}

/// Dependency counters for a compose service.
///
/// Replaces the deleted `crate::repository::ResourceDependencyCounts`
/// (same shape and `blocks_delete` semantics). Kept local to the compose
/// tree so it stays decoupled from the other in-flight service migrations.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ComposeDependencyCounts {
    pub active_deployments: i64,
    pub deployments: i64,
    pub domains: i64,
    pub mounts: i64,
    pub patches: i64,
    pub schedules: i64,
    pub backups: i64,
    pub enabled_backups: i64,
    pub preview_deployments: i64,
    pub middleware: i64,
    pub ports: i64,
    pub redirects: i64,
    pub security_rules: i64,
}

impl ComposeDependencyCounts {
    pub fn blocks_delete(&self) -> bool {
        self.active_deployments > 0 || self.enabled_backups > 0
    }
}

pub(super) async fn compose_dependency_counts(
    queries: &FlashQueries,
    service_id: i64,
) -> sqlx::Result<ComposeDependencyCounts> {
    let key = service_id.to_string();
    let params = ServiceDependencyCountsParams {
        service_id: key.clone(),
        service_id2: key.clone(),
        service_id3: key.clone(),
        service_id4: key.clone(),
        service_id5: key.clone(),
        service_id6: key.clone(),
        service_id7: key.clone(),
        service_id8: key.clone(),
        base_service_id: key.clone(),
        preview_service_id: key.clone(),
        service_id9: key.clone(),
        service_id10: key.clone(),
        service_id11: key.clone(),
        service_id12: key,
    };
    let row = queries.service_dependency_counts(&params).await?;
    Ok(ComposeDependencyCounts {
        active_deployments: row.active_deployments,
        deployments: row.deployments,
        domains: row.domains,
        mounts: row.mounts,
        patches: row.patches,
        schedules: row.schedules,
        backups: row.backups,
        enabled_backups: row.enabled_backups,
        preview_deployments: row.preview_deployments,
        middleware: row.middleware,
        ports: row.ports,
        redirects: row.redirects,
        security_rules: row.security_rules,
    })
}

/// Returns the persistent Agent-backed executor used for normal server
/// operations. SSH remains limited to bootstrap/setup and explicit terminal
/// sessions; deployments and Docker commands must use the Agent gRPC channel.
pub(crate) async fn agent_executor(
    server_id: i64,
) -> Result<crate::utils::exec::CommandExecutor, String> {
    let monitoring =
        resolve::<crate::services::observability::monitoring::service::MonitoringService>()
            .await
            .map_err(|error| format!("monitoring Agent unavailable: {error}"))?;
    Ok(crate::utils::exec::CommandExecutor::Agent(
        monitoring.agent_executor(server_id),
    ))
}

/// Resolve an optional resource server to an Agent identity. Resources with
/// no explicit server belong to the panel host, which is represented by the
/// local Agent registration just like every remote host.
pub(crate) async fn agent_server_id(server_id: Option<i64>) -> Result<i64, String> {
    if let Some(server_id) = server_id {
        return Ok(server_id);
    }
    let lifecycle = resolve::<
        crate::services::observability::monitoring::lifecycle::MonitoringLifecycleService,
    >()
    .await
    .map_err(|error| format!("local Agent lifecycle unavailable: {error}"))?;
    lifecycle
        .ensure_local_registration()
        .await
        .map_err(|error| format!("local Agent registration failed: {error}"))
}

pub async fn agent_executor_for(
    server_id: Option<i64>,
) -> Result<crate::utils::exec::CommandExecutor, String> {
    agent_executor(agent_server_id(server_id).await?).await
}

pub(crate) fn deployment_pid_file(deployment_id: i64) -> String {
    format!("/tmp/openoxide-deployment-{deployment_id}.pid")
}

/// Signature preserved verbatim from the repository era; the unused `db`
/// handle is accepted for compatibility while the FlashQueries client is
/// resolved from DI.
pub(crate) async fn remote_executor(
    _db: &SqlitePool,
    server_id: i64,
) -> Result<RemoteExecutor, String> {
    let queries = resolve::<FlashQueries>()
        .await
        .map_err(|error| format!("database queries unavailable: {error}"))?;
    let (host, port, username, private_key, public_key) =
        crate::services::infrastructure::server::ssh_credentials(&queries, server_id)
            .await
            .map_err(|error| format!("could not load SSH credentials: {error}"))?
            .ok_or_else(|| "Server not found".to_string())?;

    let port = u16::try_from(port).map_err(|_| "SSH port must be between 0 and 65535")?;
    let host_key = match queries.servers_get_host_key_fingerprint(server_id).await {
        Ok(fp) if !fp.trim().is_empty() => SshHostKey::PinnedSha256(fp.trim().to_owned()),
        _ => {
            tracing::warn!(
                server_id,
                "compose deployment SSH host key verification is disabled because no fingerprint is stored for this server"
            );
            SshHostKey::InsecureAcceptAny
        }
    };
    let mut executor = RemoteExecutor::new(
        host,
        port,
        username.clone(),
        SshAuth::key_pair(private_key, public_key),
        host_key,
    );
    if username != "root" {
        executor = executor.with_sudo();
    }
    Ok(executor)
}
