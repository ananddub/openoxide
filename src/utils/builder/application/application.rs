use crate::utils::builder::application::{stack::stack_spec, validation::validate_spec};
use crate::utils::builder::shared::BuilderContext;
use crate::utils::builder::spec::{ApplicationSpec, BuilderEvent, DeploymentResult};
use crate::utils::builder::swarm::{ensure_overlay_network, ensure_swarm_manager};
use crate::utils::{
    exec::{CommandExecutor, ExecResult},
    paths::rustploy_paths,
};
use os::pipeline;
use tokio_util::sync::CancellationToken;

#[derive(Clone, Debug)]
pub struct ApplicationBuilder {
    pub(super) ctx: BuilderContext,
}

impl ApplicationBuilder {
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
        spec: &ApplicationSpec,
        cancel: &CancellationToken,
    ) -> ExecResult<DeploymentResult> {
        validate_spec(spec)?;

        self.ctx.emit(BuilderEvent::Preparing).await;
        self.ctx.cancelled(cancel)?;
        self.prepare_source(spec, cancel).await?;
        self.apply_patches(spec, cancel).await?;
        self.ctx.emit(BuilderEvent::SourceReady).await;

        self.ctx.cancelled(cancel)?;
        self.ctx.emit(BuilderEvent::Building).await;
        self.build_image(spec, cancel).await?;
        self.ctx.emit(BuilderEvent::ImageReady).await;
        let deployed_image = self.distribute_image(spec, cancel).await?;

        self.ctx.cancelled(cancel)?;
        let paths = rustploy_paths();
        let app_dir = paths.application_dir(&spec.app_name);

        self.prepare_file_mounts(spec, cancel).await?;
        ensure_swarm_manager(&self.ctx.executor, &self.ctx.docker, cancel).await?;
        for network in &spec.networks {
            ensure_overlay_network(&self.ctx.docker, network.as_str(), cancel).await?;
        }

        let stack_file = format!("{app_dir}/stack.yml");
        let mut deployed_spec = spec.clone();
        deployed_spec.image = deployed_image.clone();
        let stack_yaml = serde_yaml::to_string(&stack_spec(&deployed_spec)).map_err(|e| {
            crate::utils::exec::ExecError::Json(serde_json::Error::io(std::io::Error::other(e)))
        })?;

        self.ctx.emit(BuilderEvent::Deploying).await;

        let mkdir_cmd = format!(
            "mkdir -p {}",
            crate::utils::exec::script::shell_single_quote(&app_dir)
        );
        let write_yaml_cmd = format!(
            "cat << 'EOF' > {}\n{}\nEOF",
            crate::utils::exec::script::shell_single_quote(&stack_file),
            stack_yaml
        );

        let stacks = self.ctx.docker.stacks();
        let deploy_cmd = stacks
            .deploy(spec.stack_name.clone())
            .with_registry_auth()
            .compose_file(&stack_file)
            .cancel_with(cancel.clone());

        let pipeline = pipeline! {
            mkdir_cmd;
            write_yaml_cmd;
            deploy_cmd;
        };

        if let Err(error) = pipeline.execute_cancelled(&self.ctx.executor, cancel).await {
            self.ctx
                .docker
                .services()
                .rollback(spec.service_name().as_str())
                .run()
                .await?;
            self.ctx.emit(BuilderEvent::Failed(error.to_string())).await;
            return Err(error);
        }

        self.ctx.cancelled(cancel)?;
        self.ctx.emit(BuilderEvent::Routing).await;

        self.ctx.emit(BuilderEvent::HealthCheck).await;
        if let Err(error) = self.wait_healthy(spec, cancel).await {
            self.ctx
                .docker
                .services()
                .rollback(spec.service_name().as_str())
                .run()
                .await?;
            self.ctx.emit(BuilderEvent::Failed(error.to_string())).await;
            return Err(error);
        }

        self.ctx.emit(BuilderEvent::Deployed).await;
        Ok(DeploymentResult {
            app_name: spec.app_name.clone(),
            image: deployed_image,
            service_name: spec.service_name(),
            stack_file,
        })
    }
}
