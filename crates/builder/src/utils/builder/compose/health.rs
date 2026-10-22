use super::{
    builder::ComposeBuilder,
    spec::{ComposeRuntime, ComposeSpec},
};
use crate::utils::docker::query::filter::TaskFilter;
use crate::utils::exec::{ExecError, ExecResult};
use tokio::time::{Duration, Instant};
use tokio_util::sync::CancellationToken;

impl ComposeBuilder {
    pub(super) async fn wait_healthy(
        &self,
        spec: &ComposeSpec,
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        let deadline = Instant::now() + self.ctx.health_timeout;
        loop {
            self.ctx.cancelled(cancel)?;

            let is_healthy = match spec.runtime {
                ComposeRuntime::Stack => {
                    let rows = match self
                        .ctx
                        .docker
                        .stacks()
                        .ps(&spec.stack_name)
                        .filter(TaskFilter::DesiredState(
                            crate::utils::docker::query::filter::TaskDesiredState::Running,
                        ))
                        .run_json()
                        .await
                    {
                        Ok(rows) => rows,
                        Err(error) => {
                            if Instant::now() < deadline
                                && crate::utils::docker::error::is_transient_docker_error(
                                    &error.to_string(),
                                )
                            {
                                tracing::warn!(error = %error, "compose health check failed transiently; retrying");
                                tokio::time::sleep(Duration::from_secs(2)).await;
                                continue;
                            }
                            return Err(error);
                        }
                    };

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

                    rows.iter()
                        .any(|row| row.current_state.starts_with("Running"))
                }
                ComposeRuntime::Compose => {
                    let rows = match self
                        .ctx
                        .docker
                        .compose()
                        .ps()
                        .project(&spec.stack_name)
                        .env_file(&spec.env_file)
                        .file(spec.compose_file_path())
                        .list()
                        .await
                    {
                        Ok(rows) => rows,
                        Err(error) => {
                            if Instant::now() < deadline
                                && crate::utils::docker::error::is_transient_docker_error(
                                    &error.to_string(),
                                )
                            {
                                tracing::warn!(error = %error, "compose health check failed transiently; retrying");
                                tokio::time::sleep(Duration::from_secs(2)).await;
                                continue;
                            }
                            return Err(error);
                        }
                    };

                    // Fail fast only on a real crash — a dead container or a
                    // non-zero exit. An "unhealthy" container is NOT fatal
                    // here: slow apps (DB migrations, workers) report unhealthy
                    // during their healthcheck start period and only turn
                    // healthy once booted, so keep waiting until the deadline
                    // rather than aborting the deploy prematurely.
                    if let Some(bad) = rows.iter().find(|row| {
                        let state = row.state.to_lowercase();
                        state.contains("dead") || (state.contains("exit") && row.exit_code != 0)
                    }) {
                        return Err(ExecError::CommandFailed {
                            code: None,
                            stderr: format!(
                                "container {} failed: state={} health={} exit_code={}",
                                bad.service, bad.state, bad.health, bad.exit_code
                            ),
                        });
                    }

                    // Healthy when every container is either running (with any
                    // healthcheck passed, not "starting") or cleanly exited.
                    !rows.is_empty()
                        && rows.iter().all(|row| {
                            let state = row.state.to_lowercase();
                            (state == "running" && !row.health.eq_ignore_ascii_case("starting"))
                                || (state.contains("exit") && row.exit_code == 0)
                        })
                }
            };

            if is_healthy {
                return Ok(());
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
