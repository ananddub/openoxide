use auto_di::resolve;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::{
    services::compose::{
        ComposeOperation,
        remote::{agent_executor, deployment_pid_file},
    },
    utils::{
        builder::errors::BuilderError,
        builder::{
            compose::{ComposeBuilder, adapter::ComposeSpecAdapter},
            custom_type::IdType,
            hash_state::ApplicationState,
            queue::queue::BuilderQueue,
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
            .map_err(|e| {
                BuilderError::Execution(format!("could not resolve FlashQueries: {e}"))
            })?;
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
        state.reset_default(compose_key.clone(), environment_id, project_id);
        let cancel = state
            .cancellation_token(compose_key.clone())
            .unwrap_or_else(CancellationToken::new);

        let effective_server_id = crate::services::compose::remote::agent_server_id(server_id)
            .await
            .map_err(BuilderError::Execution)?;
        let executor = {
            let pid_file = deployment_pid_file(deployment_id);
            q.deployments_set_pid(&pid_file, deployment_id)
                .await
                .map_err(|e| {
                    BuilderError::Execution(format!(
                        "could not persist remote compose pid file: {e}"
                    ))
                })?;
            agent_executor(effective_server_id)
                .await
                .map_err(BuilderError::Execution)?
        };

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
