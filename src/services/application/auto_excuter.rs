use crate::repository::ApplicationRepository;
use crate::utils::exec::{CommandExecutor, LocalExecutor, RemoteExecutor};
use auto_di::resolve;
use sqlx::SqlitePool;
use std::sync::Arc;

pub async fn app_new_cmd(
    _db: Arc<SqlitePool>,
    app_id: i64,
) -> Result<CommandExecutor, sqlx::Error> {
    let repo = resolve::<ApplicationRepository>().await.unwrap();
    let app_user = repo
        .get_by_id(app_id)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;
    let cmd: CommandExecutor;
    if app_user.server_id.is_none() {
        cmd = CommandExecutor::Local(LocalExecutor::new());
        tracing::warn!(
            application_id = app_user.id,
            "application has no server assigned; cannot cancel operation"
        );
    } else {
        let server_repo = resolve::<crate::repository::ServerRepository>()
            .await
            .unwrap();
        let server_id = app_user.server_id.unwrap_or(0);
        let creds = server_repo
            .get_ssh_credentials(server_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        let port = u16::try_from(creds.1).map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
        let rm = RemoteExecutor::new(
            creds.0,
            port,
            creds.2,
            crate::utils::exec::SshAuth::key_pair(creds.3, creds.4),
            crate::utils::exec::SshHostKey::InsecureAcceptAny,
        )
        .with_sudo();
        cmd = CommandExecutor::Remote(rm)
    }
    Ok(cmd)
}
