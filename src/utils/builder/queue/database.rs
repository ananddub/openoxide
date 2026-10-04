use auto_di::resolve;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::{
    services::compute::database::DatabaseOperation,
    utils::{
        builder::errors::BuilderError,
        builder::{
            custom_type::IdType, database::builder::DatabaseBuilder, hash_state::ApplicationState,
            queue::dispatcher::BuilderQueue,
        },
    },
};

impl BuilderQueue {
    pub(crate) async fn execute_operation_db(
        &self,
        database_id: i64,
        database_kind: String,
        deployment_id: i64,
        operation: DatabaseOperation,
    ) -> Result<(), BuilderError> {
        let kind = std::str::FromStr::from_str(&database_kind).map_err(|_| {
            BuilderError::Execution(format!("Invalid database kind: {}", database_kind))
        })?;

        let q = resolve::<crate::db::queries::FlashQueries>()
            .await
            .map_err(|e| BuilderError::Execution(format!("could not resolve FlashQueries: {e}")))?;

        let ctx = q
            .database_get_deployment_context(database_id)
            .await
            .map_err(|e| BuilderError::Execution(format!("Database context not found: {}", e)))?;
        let (server_id, environment_id, project_id) =
            (ctx.server_id, ctx.environment_id, ctx.project_id);

        let db_key = IdType::DatabaseId(database_id);
        let state = resolve::<ApplicationState>().await.map_err(|e| {
            BuilderError::Execution(format!("could not resolve application state: {e}"))
        })?;
        state.reset_default(db_key, environment_id, project_id);
        let cancel = state
            .cancellation_token(db_key)
            .unwrap_or_else(CancellationToken::new);

        let executor = self
            .executor_for_deployment(server_id, deployment_id)
            .await?;

        let (events_tx, events_rx) = mpsc::channel(6);
        tokio::spawn(super::deployment_log::record_builder_events(
            deployment_id,
            events_rx,
            "db",
        ));

        let events_tx_clone = events_tx.clone();

        let build_future = async move {
            let builder = DatabaseBuilder::new(executor)
                .with_state(state, db_key)
                .with_events(events_tx);

            match operation {
                DatabaseOperation::Deploy
                | DatabaseOperation::Redeploy
                | DatabaseOperation::Start => builder
                    .deploy(kind, database_id, &cancel)
                    .await
                    .map(|_| ())
                    .map_err(|e| BuilderError::Execution(e.to_string())),
                DatabaseOperation::Reload => {
                    let app_name = q.services_get_app_name(database_id).await.map_err(|e| {
                        BuilderError::Execution(format!("Failed to query database app_name: {e}"))
                    })?;
                    builder
                        .reload(&app_name, &cancel)
                        .await
                        .map_err(|e| BuilderError::Execution(e.to_string()))
                }
                DatabaseOperation::Stop => {
                    let app_name = q.services_get_app_name(database_id).await.map_err(|e| {
                        BuilderError::Execution(format!("Failed to query database app_name: {}", e))
                    })?;

                    builder
                        .stop(&app_name, &cancel)
                        .await
                        .map(|_| ())
                        .map_err(|e| BuilderError::Execution(e.to_string()))
                }
            }
        };

        super::deployment_log::DEPLOYMENT_SENDER
            .scope(events_tx_clone, build_future)
            .await
    }
}
