use super::{
    labels::write_labeled_compose,
    spec::{ComposeDeploymentResult, ComposeRuntime, ComposeSpec},
    validation::validate_spec,
};
use crate::utils::docker::core::types::ResolveImage;
use crate::utils::{
    builder::{
        shared::BuilderContext,
        spec::BuilderEvent,
        swarm::{OPENOXIDE_NETWORK, ensure_overlay_network, ensure_swarm_manager},
    },
    exec::{CommandExecutor, ExecResult},
};
use os::exec::IntoCommand;
use os::pipeline;
use tokio_util::sync::CancellationToken;

#[derive(Clone, Debug)]
pub struct ComposeBuilder {
    pub(super) ctx: BuilderContext,
}

impl ComposeBuilder {
    pub fn new(executor: CommandExecutor) -> Self {
        Self {
            ctx: BuilderContext::new(executor),
        }
    }

    pub fn with_events(mut self, events: tokio::sync::mpsc::Sender<BuilderEvent>) -> Self {
        self.ctx = self.ctx.with_events(events);
        self
    }

    pub fn with_state(
        mut self,
        state: std::sync::Arc<crate::utils::builder::hash_state::ApplicationState>,
        id: crate::utils::builder::custom_type::IdType,
    ) -> Self {
        self.ctx = self.ctx.with_state(state, id);
        self
    }

    pub fn with_health_timeout(mut self, timeout: tokio::time::Duration) -> Self {
        self.ctx = self.ctx.with_health_timeout(timeout);
        self
    }

    pub async fn deploy(
        &self,
        spec: &ComposeSpec,
        cancel: &CancellationToken,
    ) -> ExecResult<ComposeDeploymentResult> {
        validate_spec(spec)?;
        self.ctx.emit(BuilderEvent::Preparing).await;
        self.ctx.cancelled(cancel)?;
        self.prepare_source(spec, cancel).await?;
        crate::utils::builder::shared::patches::apply_file_patches(
            &self.ctx,
            &spec.work_directory,
            &spec.patches,
            cancel,
        )
        .await?;
        self.transform_compose_file(spec, cancel).await?;
        self.ctx.emit(BuilderEvent::SourceReady).await;
        self.prepare_runtime_files(spec, cancel).await?;
        write_labeled_compose(self, spec, cancel).await?;

        self.ctx.emit(BuilderEvent::Deploying).await;
        let deploy_result = match spec.runtime {
            ComposeRuntime::Stack => self.deploy_stack(spec, cancel).await,
            ComposeRuntime::Compose => self.deploy_compose(spec, cancel).await,
        };
        if let Err(error) = deploy_result {
            self.cleanup_failed_deploy(spec).await;
            self.ctx.emit(BuilderEvent::Failed(error.to_string())).await;
            return Err(error);
        }

        self.ctx.emit(BuilderEvent::Routing).await;

        self.ctx.emit(BuilderEvent::HealthCheck).await;
        self.ctx
            .emit(BuilderEvent::Message(format!(
                "Waiting for {} to become healthy...",
                match spec.runtime {
                    ComposeRuntime::Stack => format!("stack {}", spec.stack_name),
                    ComposeRuntime::Compose => format!("compose project {}", spec.stack_name),
                }
            )))
            .await;
        if let Err(error) = self.wait_healthy(spec, cancel).await {
            self.cleanup_failed_deploy(spec).await;
            self.ctx.emit(BuilderEvent::Failed(error.to_string())).await;
            return Err(error);
        }
        self.ctx
            .emit(BuilderEvent::Message(format!(
                "Deployment {} is running",
                spec.stack_name
            )))
            .await;

        self.ctx.emit(BuilderEvent::Deployed).await;
        Ok(ComposeDeploymentResult {
            app_name: spec.app_name.clone(),
            stack_name: spec.stack_name.clone(),
            compose_file: spec.compose_file_path(),
        })
    }

    pub async fn stop(&self, spec: &ComposeSpec) -> ExecResult<()> {
        match spec.runtime {
            ComposeRuntime::Stack => {
                let _ = self
                    .ctx
                    .docker
                    .stacks()
                    .remove(&spec.stack_name)
                    .run()
                    .await;
            }
            ComposeRuntime::Compose => {
                let primary_down = self
                    .ctx
                    .docker
                    .compose()
                    .down()
                    .project(&spec.stack_name)
                    .remove_orphans()
                    .run()
                    .await;

                if primary_down.is_err() {
                    let _ = self
                        .ctx
                        .docker
                        .compose()
                        .stop()
                        .project(&spec.stack_name)
                        .run()
                        .await;
                }
            }
        }
        self.ctx.emit(BuilderEvent::Cancelled).await;
        Ok(())
    }

    async fn deploy_stack(&self, spec: &ComposeSpec, cancel: &CancellationToken) -> ExecResult<()> {
        ensure_swarm_manager(&self.ctx.executor, &self.ctx.docker, cancel).await?;
        ensure_overlay_network(&self.ctx.docker, OPENOXIDE_NETWORK, cancel).await?;
        for service_network in &spec.service_networks {
            for network in &service_network.networks {
                ensure_overlay_network(&self.ctx.docker, network, cancel).await?;
            }
        }
        if spec.isolated_deployment {
            ensure_overlay_network(&self.ctx.docker, &spec.app_name, cancel).await?;
        }
        self.ctx
            .emit(BuilderEvent::Message(format!(
                "building compose stack {} from {}",
                spec.stack_name,
                spec.compose_file_path()
            )))
            .await;

        let compose = self.ctx.docker.compose();
        let build_cmd = compose
            .build()
            .project(&spec.stack_name)
            .env_file(&spec.env_file)
            .file(&spec.compose_file_path())
            .retry(3)
            .cancel_with(cancel.clone());

        let config_builder = compose
            .config()
            .env_file(&spec.env_file)
            .file(&spec.compose_file_path())
            .retry(3)
            .cancel_with(cancel.clone());

        // `file().write()` dumps its argument's text into the file, so the config
        // command must actually run first and its stdout be written back.
        let config_output = config_builder
            .execute_cancelled(&self.ctx.executor, cancel)
            .await?;
        let rendered = strip_project_name(&config_output.stdout);
        self.ctx
            .write_file_cancelled(&spec.rendered_stack_file, rendered.as_bytes(), cancel)
            .await?;

        let stacks = self.ctx.docker.stacks();
        let deploy_cmd = stacks
            .deploy(&spec.stack_name)
            .compose_file(&spec.rendered_stack_file)
            .with_registry_auth()
            .resolve_image(ResolveImage::Never)
            .retry(3)
            .cancel_with(cancel.clone());

        let pipeline = pipeline! {
            build_cmd;
            deploy_cmd;
        };

        let output = pipeline
            .execute_cancelled(&self.ctx.executor, cancel)
            .await?;
        if !output.stdout.is_empty() {
            self.ctx
                .emit(BuilderEvent::Message(output.stdout.clone()))
                .await;
        }
        if !output.stderr.is_empty() {
            self.ctx
                .emit(BuilderEvent::Message(output.stderr.clone()))
                .await;
        }
        Ok(())
    }

    async fn deploy_compose(
        &self,
        spec: &ComposeSpec,
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        if spec.isolated_deployment
            && self
                .ctx
                .docker
                .networks()
                .inspect(&spec.app_name)
                .await
                .is_err()
        {
            self.ctx
                .docker
                .networks()
                .create(&spec.app_name)
                .driver(crate::utils::docker::NetworkDriver::Bridge)
                .attachable()
                .run()
                .await?;
        }
        self.ctx
            .emit(BuilderEvent::Message(format!(
                "docker compose build and up project {} file {}",
                spec.stack_name,
                spec.compose_file_path()
            )))
            .await;

        let compose = self.ctx.docker.compose();
        let build_cmd = compose
            .build()
            .project(&spec.stack_name)
            .env_file(&spec.env_file)
            .file(&spec.compose_file_path())
            .retry(3)
            .cancel_with(cancel.clone());

        let up_cmd = compose
            .up()
            .project(&spec.stack_name)
            .env_file(&spec.env_file)
            .file(&spec.compose_file_path())
            .detach()
            .retry(3)
            .cancel_with(cancel.clone());

        let pipeline = pipeline! {
            build_cmd;
            up_cmd;
        };

        let output = pipeline
            .execute_cancelled(&self.ctx.executor, cancel)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "docker compose deploy failed");
                e
            })?;
        if !output.stdout.is_empty() {
            self.ctx
                .emit(BuilderEvent::Message(output.stdout.clone()))
                .await;
        }
        if !output.stderr.is_empty() {
            self.ctx
                .emit(BuilderEvent::Message(output.stderr.clone()))
                .await;
        }
        Ok(())
    }

    async fn cleanup_failed_deploy(&self, spec: &ComposeSpec) {
        match spec.runtime {
            ComposeRuntime::Stack => {
                if let Err(error) = self
                    .ctx
                    .docker
                    .stacks()
                    .remove(&spec.stack_name)
                    .run()
                    .await
                {
                    tracing::warn!(stack = %spec.stack_name, error = %error, "compose stack cleanup failed");
                }
            }
            ComposeRuntime::Compose => {
                if let Err(error) = self
                    .ctx
                    .docker
                    .compose()
                    .down()
                    .project(&spec.stack_name)
                    .env_file(&spec.env_file)
                    .file(&spec.compose_file_path())
                    .retry(3)
                    .run()
                    .await
                {
                    tracing::warn!(compose = %spec.stack_name, error = %error, "compose cleanup failed");
                }
            }
        }
    }
}

/// `docker compose config` prefixes its output with a top-level `name:` key,
/// which `docker stack deploy` rejects ("(root) Additional property name is not
/// allowed"). The stack name is already the deploy argument, so drop it. Nested
/// `name:` keys are indented and therefore survive.
fn strip_project_name(config: &str) -> String {
    config
        .lines()
        .filter(|line| !line.starts_with("name:"))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::strip_project_name;

    #[test]
    fn only_the_top_level_project_name_is_dropped() {
        let config = [
            "name: my-stack",
            "services:",
            "  db:",
            "    image: postgres",
            "networks:",
            "  default:",
            "    name: my-stack_default",
        ]
        .join("\n");
        let rendered = strip_project_name(&config);
        assert!(!rendered.starts_with("name:"));
        assert!(rendered.contains("services:"));
        assert!(rendered.contains("    name: my-stack_default"));
    }
}
