use sqlx::SqlitePool;

use crate::db::queries::FlashQueries;
use crate::services::server::ssh_credentials;
use crate::utils::exec::{RemoteExecutor, SshAuth, SshHostKey};
use auto_di::resolve;

pub(crate) async fn remote_executor(
    _db: &SqlitePool,
    server_id: i64,
) -> Result<RemoteExecutor, String> {
    let queries = resolve::<FlashQueries>()
        .await
        .map_err(|error| format!("could not resolve database queries: {error}"))?;
    let (host, port, username, private_key, public_key) = ssh_credentials(&queries, server_id)
        .await
        .map_err(|error| format!("could not load SSH credentials: {error}"))?
        .ok_or_else(|| "Server not found".to_string())?;

    let port = u16::try_from(port).map_err(|_| "SSH port must be between 0 and 65535")?;
    tracing::warn!(
        server_id,
        "deployment SSH host key verification is disabled because no fingerprint is stored for this server"
    );
    let mut executor = RemoteExecutor::new(
        host,
        port,
        username.clone(),
        SshAuth::key_pair(private_key, public_key),
        SshHostKey::InsecureAcceptAny,
    );
    if username != "root" {
        executor = executor.with_sudo();
    }
    Ok(executor)
}
