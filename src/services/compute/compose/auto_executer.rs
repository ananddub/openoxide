use crate::db::queries::FlashQueries;
use crate::utils::exec::CommandExecutor;
use auto_di::resolve;

/// `db` is retained for signature compatibility with the caller in
/// `operations.rs`; the FlashQueries client is resolved from DI.
pub async fn compose_new_db(
    _db: sqlx::SqlitePool,
    compose_id: i64,
) -> Result<CommandExecutor, sqlx::Error> {
    let queries = resolve::<FlashQueries>()
        .await
        .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
    let service = queries.get_service_by_i_d(compose_id).await?;
    crate::services::compute::compose::remote::agent_executor_for(service.server_id)
        .await
        .map_err(sqlx::Error::Protocol)
}
