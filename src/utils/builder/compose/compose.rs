use super::{
    labels::write_labeled_compose,
    spec::{ComposeDeploymentResult, ComposeRuntime, ComposeSpec},
    validation::validate_spec,
};
use crate::pipeline;
use crate::utils::docker::core::types::ResolveImage;
use crate::utils::exec::script::IntoCommand;
use crate::utils::{
    builder::{
        shared::BuilderContext,
        spec::BuilderEvent,
        swarm::{RUSTPLOY_NETWORK, ensure_overlay_network, ensure_swarm_manager},
    },
    exec::{CommandExecutor, ExecResult},
};
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

    pub fn with_cgroup(mut self, cg: crate::utils::cgroup::Cgroup) -> Self {
        self.ctx = self.ctx.with_cgroup(cg);
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
        if let Err(error) = self.wait_healthy(spec, cancel).await {
            self.cleanup_failed_deploy(spec).await;
            self.ctx.emit(BuilderEvent::Failed(error.to_string())).await;
            return Err(error);
        }

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
                self.ctx
                    .docker
                    .stacks()
                    .remove(&spec.stack_name)
                    .run()
                    .await?;
            }
            ComposeRuntime::Compose => {
                self.ctx
                    .docker
                    .compose()
                    .down()
                    .project(&spec.stack_name)
                    .env_file(&spec.env_file)
                    .file(&spec.compose_file_path())
                    .retry(3)
                    .run()
                    .await?;
            }
        }
        self.ctx.emit(BuilderEvent::Cancelled).await;
        Ok(())
    }

    async fn deploy_stack(&self, spec: &ComposeSpec, cancel: &CancellationToken) -> ExecResult<()> {
        ensure_swarm_manager(&self.ctx.executor, &self.ctx.docker, cancel).await?;
        ensure_overlay_network(&self.ctx.docker, RUSTPLOY_NETWORK, cancel).await?;
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

        let config_cmd = format!(
            "{} > {}",
            compose
                .config()
                .env_file(&spec.env_file)
                .file(&spec.compose_file_path())
                .retry(3)
                .cancel_with(cancel.clone())
                .build_str(),
            crate::utils::exec::script::shell_single_quote(&spec.rendered_stack_file)
        );

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
            config_cmd;
            deploy_cmd;
        };

        self.ctx
            .apply_cgroup(pipeline)?
            .execute_cancelled(&self.ctx.executor, cancel)
            .await?;
        Ok(())
    }

    async fn deploy_compose(
        &self,
        spec: &ComposeSpec,
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
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

        self.ctx
            .apply_cgroup(pipeline)?
            .execute_cancelled(&self.ctx.executor, cancel)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "docker compose deploy failed");
                e
            })?;
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
