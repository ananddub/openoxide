use sqlx::SqlitePool;

use crate::repository::ServerRepository;
use crate::utils::exec::{RemoteExecutor, SshAuth, SshHostKey};
use auto_di::resolve;

/// Returns the persistent Agent-backed executor used for normal server
/// operations. SSH remains limited to bootstrap/setup and explicit terminal
/// sessions; deployments and Docker commands must use the Agent gRPC channel.
pub(crate) async fn agent_executor(
    server_id: i64,
) -> Result<crate::utils::exec::CommandExecutor, String> {
    let monitoring =
        resolve::<crate::services::monitoring::monitoring_service::MonitoringService>()
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
    let lifecycle = resolve::<crate::services::monitoring::lifecycle::MonitoringLifecycleService>()
        .await
        .map_err(|error| format!("local Agent lifecycle unavailable: {error}"))?;
    lifecycle
        .ensure_local_registration()
        .await
        .map_err(|error| format!("local Agent registration failed: {error}"))
}

pub(crate) async fn agent_executor_for(
    server_id: Option<i64>,
) -> Result<crate::utils::exec::CommandExecutor, String> {
    agent_executor(agent_server_id(server_id).await?).await
}

pub(crate) fn deployment_pid_file(deployment_id: i64) -> String {
    format!("/tmp/openoxide-deployment-{deployment_id}.pid")
}

pub(crate) async fn remote_executor(
    _db: &SqlitePool,
    server_id: i64,
) -> Result<RemoteExecutor, String> {
    let repo_servers = resolve::<ServerRepository>().await.unwrap();
    let row = repo_servers
        .get_ssh_credentials(server_id)
        .await
        .map_err(|error| format!("could not load SSH credentials: {error}"))?
        .ok_or_else(|| "Server not found".to_string())?;

    let port = u16::try_from(row.1).map_err(|_| "SSH port must be between 0 and 65535")?;
    tracing::warn!(
        server_id,
        "compose deployment SSH host key verification is disabled because no fingerprint is stored for this server"
    );
    let username = row.2;
    let mut executor = RemoteExecutor::new(
        row.0,
        port,
        username.clone(),
        SshAuth::key_pair(row.3, row.4),
        SshHostKey::InsecureAcceptAny,
    );
    if username != "root" {
        executor = executor.with_sudo();
    }
    Ok(executor)
}
