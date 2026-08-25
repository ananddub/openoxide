use crate::repository::ComposeProjectRepository;
use crate::utils::exec::CommandExecutor;
use auto_di::resolve;
use sqlx::SqlitePool;
use std::sync::Arc;

pub async fn compose_new_db(
    _db: Arc<SqlitePool>,
    compose_id: i64,
) -> Result<CommandExecutor, sqlx::Error> {
    let repo = resolve::<ComposeProjectRepository>().await.unwrap();
    let compose_user = repo
        .get_by_id(compose_id)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;
    crate::services::compose::remote::agent_executor_for(compose_user.server_id)
        .await
        .map_err(sqlx::Error::Protocol)
}
