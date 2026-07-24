use crate::repository::ApplicationRepository;
use crate::utils::exec::{CommandExecutor, LocalExecutor, RemoteExecutor};
use auto_di::resolve;
use sqlx::SqlitePool;
use std::sync::Arc;

pub async fn app_new_cmd(db: Arc<SqlitePool>, app_id: i64) -> Result<CommandExecutor, sqlx::Error> {
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
        let rm = RemoteExecutor::new_with_db(db.clone(), app_user.server_id.unwrap_or(0)).await?;
        cmd = CommandExecutor::Remote(rm)
    }
    Ok(cmd)
}
