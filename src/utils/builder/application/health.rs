use super::application::ApplicationBuilder;
use crate::utils::{
    builder::spec::ApplicationSpec,
    docker::query::filter::{TaskDesiredState, TaskFilter},
    exec::{ExecError, ExecResult},
};
use tokio::time::{Duration, Instant};
use tokio_util::sync::CancellationToken;

impl ApplicationBuilder {
    pub(super) async fn wait_healthy(
        &self,
        spec: &ApplicationSpec,
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        let deadline = Instant::now() + self.ctx.health_timeout;
        loop {
            self.ctx.cancelled(cancel)?;
            let health_result = self
                .ctx
                .docker
                .services()
                .ps(spec.service_name())
                .filter(TaskFilter::DesiredState(TaskDesiredState::Running))
                .run_json()
                .await;
            let rows = match health_result {
                Ok(rows) => rows,
                Err(error)
                    if Instant::now() < deadline
                        && crate::utils::docker::error::is_transient_docker_error(
                            &error.to_string(),
                        ) =>
                {
                    tracing::warn!(error = %error, "docker service health check failed transiently; retrying");
                    tokio::time::sleep(Duration::from_secs(2)).await;
                    continue;
                }
                Err(error) => return Err(error),
            };
            if rows
                .iter()
                .any(|row| row.current_state.starts_with("Running"))
            {
                return Ok(());
            }
            if let Some(error) = rows
                .iter()
                .map(|row| row.error.as_str())
                .find(|e| !e.is_empty())
            {
                return Err(ExecError::CommandFailed {
                    code: None,
                    stderr: error.into(),
                });
            }
            if Instant::now() >= deadline {
                return Err(ExecError::Timeout {
                    seconds: self.ctx.health_timeout.as_secs(),
                });
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    }
}
