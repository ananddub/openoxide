use crate::repository::ComposeProjectRepository;
use crate::utils::exec::{CommandExecutor, LocalExecutor, RemoteExecutor};
use auto_di::resolve;
use sqlx::SqlitePool;
use std::sync::Arc;

pub async fn compose_new_db(
    db: Arc<SqlitePool>,
    compose_id: i64,
) -> Result<CommandExecutor, sqlx::Error> {
    let repo = resolve::<ComposeProjectRepository>().await.unwrap();
    let compose_user = repo
        .get_by_id(compose_id)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;
    let cmd: CommandExecutor;
    if compose_user.server_id.is_none() {
        cmd = CommandExecutor::Local(LocalExecutor::new());
        tracing::warn!(
            application_id = compose_user.id,
            "application has no server assigned; cannot cancel operation"
        );
    } else {
        let rm =
            RemoteExecutor::new_with_db(db.clone(), compose_user.server_id.unwrap_or(0)).await?;
        cmd = CommandExecutor::Remote(rm)
    }
    Ok(cmd)
}
