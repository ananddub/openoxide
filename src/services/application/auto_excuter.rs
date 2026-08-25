use crate::repository::ApplicationRepository;
use crate::utils::exec::CommandExecutor;
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
    crate::services::compose::remote::agent_executor_for(app_user.server_id)
        .await
        .map_err(sqlx::Error::Protocol)
}
