use sqlx::SqlitePool;

use crate::repository::ServerRepository;
use crate::utils::exec::{RemoteExecutor, SshAuth, SshHostKey};
use auto_di::resolve;

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
    let executor = RemoteExecutor::new(
        row.0,
        port,
        row.2,
        SshAuth::key_pair(row.3, row.4),
        SshHostKey::InsecureAcceptAny,
    )
    .with_sudo();
    Ok(executor)
}
