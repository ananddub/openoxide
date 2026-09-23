use super::application::ApplicationBuilder;
use crate::utils::builder::shared::source::fetch_git_repository;
use crate::utils::{
    builder::spec::{ApplicationSpec, BuilderEvent, SourceSpec},
    exec::{ExecError, ExecResult},
    git::GitCli,
};
use tokio_util::sync::CancellationToken;

impl ApplicationBuilder {
    pub(super) async fn prepare_source(
        &self,
        spec: &ApplicationSpec,
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        match &spec.source {
            SourceSpec::Docker { image, registry } => {
                self.pull_docker_image(image, registry, cancel).await
            }
            SourceSpec::Git {
                url,
                branch,
                submodules,
                protocol,
                auth,
            } => {
                self.fetch_git_repository(
                    spec,
                    url,
                    branch,
                    *submodules,
                    protocol.clone(),
                    auth.clone(),
                    cancel,
                )
                .await
            }
            // The drop-upload flow already extracted the ZIP into the work
            // directory before the build starts; nothing to fetch.
            SourceSpec::Drop => Ok(()),
        }
    }

    async fn pull_docker_image(
        &self,
        image: &str,
        registry: &Option<crate::utils::builder::spec::RegistryAuth>,
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        self.ctx
            .emit(BuilderEvent::Message(format!(
                "Pulling image {image}..."
            )))
            .await;
        if let Some(auth) = registry {
            self.ctx
                .docker
                .system()
                .login()
                .registry(&auth.registry)
                .username(&auth.username)
                .password(&auth.password)
                .run()
                .await?;
            let pull = self
                .ctx
                .docker
                .images()
                .pull(image)
                .cancel_with(cancel.clone())
                .pull()
                .await;
            let logout = self
                .ctx
                .docker
                .system()
                .logout()
                .registry(&auth.registry)
                .run()
                .await;
            match (pull, logout) {
                (Err(error), _) => return Err(error),
                (Ok(_), Err(error)) => return Err(error),
                (Ok(output), Ok(_)) => {
                    self.emit_pull_output(&output).await;
                    Ok(())
                }
            }
        } else {
            let output = self
                .ctx
                .docker
                .images()
                .pull(image)
                .cancel_with(cancel.clone())
                .pull()
                .await?;
            self.emit_pull_output(&output).await;
            Ok(())
        }
    }

    async fn emit_pull_output(&self, output: &crate::utils::docker::DockerOutput) {
        if !output.stdout.trim().is_empty() {
            self.ctx
                .emit(BuilderEvent::Message(output.stdout.clone()))
                .await;
        }
        if !output.stderr.trim().is_empty() {
            self.ctx
                .emit(BuilderEvent::Message(output.stderr.clone()))
                .await;
        }
    }

    async fn fetch_git_repository(
        &self,
        spec: &ApplicationSpec,
        url: &str,
        branch_opt: &Option<String>,
        submodules: bool,
        protocol: crate::utils::provider::CloneProtocol,
        auth: Option<crate::utils::git::types::GitAuth>,
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        let branch = resolve_branch(
            &GitCli::from_executor(self.ctx.executor.clone()),
            url,
            branch_opt,
            auth.clone(),
            cancel,
        )
        .await?;

        fetch_git_repository(
            &self.ctx,
            &spec.work_directory,
            url,
            &branch,
            submodules,
            protocol,
            auth,
            cancel,
        )
        .await
    }
}

async fn resolve_branch(
    git: &GitCli,
    url: &str,
    branch: &Option<String>,
    auth: Option<crate::utils::git::types::GitAuth>,
    cancel: &CancellationToken,
) -> Result<String, ExecError> {
    if let Some(branch) = branch.as_deref().map(str::trim).filter(|v| !v.is_empty()) {
        return Ok(branch.to_owned());
    }
    git.remote_default_branch_cancelled(url, auth, cancel)
        .await?
        .ok_or_else(|| ExecError::CommandFailed {
            code: None,
            stderr: format!("could not detect default branch for repository {url}"),
        })
}
