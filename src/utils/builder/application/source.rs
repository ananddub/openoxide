use super::application::ApplicationBuilder;
use crate::utils::builder::shared::source::fetch_git_repository;
use crate::utils::{
    builder::spec::{ApplicationSpec, SourceSpec},
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
        }
    }

    async fn pull_docker_image(
        &self,
        image: &str,
        registry: &Option<crate::utils::builder::spec::RegistryAuth>,
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        if let Some(auth) = registry {
            self.ctx
                .docker
                .login(Some(&auth.registry), &auth.username, &auth.password)
                .await?;
            let pull = self
                .ctx
                .docker
                .images()
                .pull(image)
                .cancel_with(cancel.clone())
                .pull()
                .await;
            let logout = self.ctx.docker.logout(Some(&auth.registry)).await;
            match (pull, logout) {
                (Err(error), _) => return Err(error),
                (Ok(_), Err(error)) => return Err(error),
                (Ok(_), Ok(_)) => Ok(()),
            }
        } else {
            self.ctx
                .docker
                .images()
                .pull(image)
                .cancel_with(cancel.clone())
                .pull()
                .await?;
            Ok(())
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
) -> ExecResult<String> {
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
