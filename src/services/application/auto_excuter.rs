use crate::db::queries::FlashQueries;
use crate::utils::exec::CommandExecutor;
use auto_di::resolve;
use sqlx::SqlitePool;
use std::sync::Arc;

pub async fn app_new_cmd(
    _db: Arc<SqlitePool>,
    app_id: i64,
) -> Result<CommandExecutor, sqlx::Error> {
    let queries = resolve::<FlashQueries>()
        .await
        .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
    let app_user = queries.get_service_by_i_d(app_id).await?;
    crate::services::compose::remote::agent_executor_for(app_user.server_id)
        .await
        .map_err(sqlx::Error::Protocol)
}
