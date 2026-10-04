use auto_di::resolve;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::{
    services::compute::compose::ComposeOperation,
    utils::{
        builder::errors::BuilderError,
        builder::{
            compose::{ComposeBuilder, adapter::ComposeSpecAdapter},
            custom_type::IdType,
            hash_state::ApplicationState,
            queue::dispatcher::BuilderQueue,
        },
    },
};

impl BuilderQueue {
    pub(crate) async fn execute_operation_compose(
        &self,
        compose_id: i64,
        deployment_id: i64,
        operation: ComposeOperation,
    ) -> Result<(), BuilderError> {
        let spec_adapter = resolve::<ComposeSpecAdapter>().await.map_err(|e| {
            BuilderError::Execution(format!("could not resolve ComposeSpecAdapter: {e}"))
        })?;
        let spec = spec_adapter
            .load(compose_id)
            .await
            .map_err(|e| BuilderError::Execution(format!("could not load compose config: {e}")))?;

        let q = resolve::<crate::db::queries::FlashQueries>()
            .await
            .map_err(|e| BuilderError::Execution(format!("could not resolve FlashQueries: {e}")))?;
        let ctx = q
            .compose_get_deployment_context(compose_id)
            .await
            .map_err(|e| {
                BuilderError::Execution(format!("could not resolve compose context: {e}"))
            })?;
        let (environment_id, project_id, server_id) =
            (ctx.environment_id, ctx.project_id, ctx.server_id);

        let compose_key = IdType::ComposeId(compose_id);
        let state = resolve::<ApplicationState>().await.map_err(|e| {
            BuilderError::Execution(format!("could not resolve application state: {e}"))
        })?;
        state.reset_default(compose_key, environment_id, project_id);
        let cancel = state
            .cancellation_token(compose_key)
            .unwrap_or_else(CancellationToken::new);

        let executor = self
            .executor_for_deployment(server_id, deployment_id)
            .await?;

        let (events_tx, events_rx) = mpsc::channel(64);
        tokio::spawn(super::deployment_log::record_builder_events(
            deployment_id,
            events_rx,
            "compose",
        ));

        let builder = ComposeBuilder::new(executor)
            .with_state(state, compose_key)
            .with_events(events_tx.clone());

        let events_tx_clone = events_tx.clone();
        let build_future = async move {
            match operation {
                ComposeOperation::Stop => builder
                    .stop(&spec)
                    .await
                    .map_err(|e| BuilderError::Execution(e.to_string())),
                ComposeOperation::Start => builder
                    .start(&spec, &cancel)
                    .await
                    .map_err(|e| BuilderError::Execution(e.to_string())),
                ComposeOperation::Reload => builder
                    .reload(&spec, &cancel)
                    .await
                    .map_err(|e| BuilderError::Execution(e.to_string())),
                // Deploy / Redeploy: full build + up.
                _ => builder
                    .deploy(&spec, &cancel)
                    .await
                    .map(|_| ())
                    .map_err(|e| BuilderError::Execution(e.to_string())),
            }
        };

        super::deployment_log::DEPLOYMENT_SENDER
            .scope(events_tx_clone, build_future)
            .await
    }
}
