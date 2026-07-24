use auto_di::resolve;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::{
    services::compose::{
        ComposeOperation,
        remote::{deployment_pid_file, remote_executor},
    },
    utils::{
        builder::errors::BuilderError,
        builder::{
            compose::{ComposeBuilder, adapter::ComposeSpecAdapter},
            custom_type::IdType,
            hash_state::ApplicationState,
            queue::queue::BuilderQueue,
        },
        exec::{CommandExecutor, LocalExecutor},
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

        let compose_repo = resolve::<crate::repository::ComposeProjectRepository>()
            .await
            .map_err(|e| {
                BuilderError::Execution(format!("could not resolve ComposeProjectRepository: {e}"))
            })?;
        let (environment_id, project_id, server_id) = compose_repo
            .get_deployment_context(compose_id)
            .await
            .map_err(|e| {
                BuilderError::Execution(format!("could not resolve compose context: {e}"))
            })?;

        let compose_key = IdType::ComposeId(compose_id);
        let state = resolve::<ApplicationState>().await.map_err(|e| {
            BuilderError::Execution(format!("could not resolve application state: {e}"))
        })?;
        state.reset_default(compose_key.clone(), environment_id, project_id);
        let cancel = state
            .cancellation_token(compose_key.clone())
            .unwrap_or_else(CancellationToken::new);

        let executor = match server_id {
            Some(sid) => {
                let pid_file = deployment_pid_file(deployment_id);
                let dep_repo = resolve::<crate::repository::DeploymentRepository>()
                    .await
                    .map_err(|e| {
                        BuilderError::Execution(format!(
                            "could not resolve DeploymentRepository: {e}"
                        ))
                    })?;
                dep_repo
                    .set_pid(deployment_id, &pid_file)
                    .await
                    .map_err(|e| {
                        BuilderError::Execution(format!(
                            "could not persist remote compose pid file: {e}"
                        ))
                    })?;
                CommandExecutor::Remote(
                    remote_executor(self.db.as_ref(), sid)
                        .await
                        .map_err(|e| BuilderError::Execution(e.to_string()))?
                        .with_job_pid_file(pid_file),
                )
            }
            None => CommandExecutor::Local(LocalExecutor::new()),
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
