use super::{compose::ComposeBuilder, spec::ComposeSource};
use crate::utils::builder::shared::source::fetch_git_repository;
use crate::utils::{builder::compose::spec::ComposeSpec, exec::ExecResult};
use tokio_util::sync::CancellationToken;

impl ComposeBuilder {
    pub(super) async fn prepare_source(
        &self,
        spec: &ComposeSpec,
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        match &spec.source {
            ComposeSource::Raw { content } => self.write_raw_source(spec, content, cancel).await,
            ComposeSource::Git {
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
            ComposeSource::Drop => Ok(()),
        }
    }

    async fn write_raw_source(
        &self,
        spec: &ComposeSpec,
        content: &str,
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        if let Some(parent) = std::path::Path::new(&spec.compose_path).parent() {
            self.ctx
                .executor
                .run_cancelled("mkdir", ["-p", parent.to_string_lossy().as_ref()], cancel)
                .await?;
        }
        self.ctx
            .write_file_cancelled(&spec.compose_path, content.as_bytes(), cancel)
            .await?;
        Ok(())
    }

    async fn fetch_git_repository(
        &self,
        spec: &ComposeSpec,
        url: &str,
        branch: &str,
        submodules: bool,
        protocol: crate::utils::provider::CloneProtocol,
        auth: Option<crate::utils::git::types::GitAuth>,
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        fetch_git_repository(
            &self.ctx,
            &spec.work_directory,
            url,
            branch,
            submodules,
            protocol,
            auth,
            cancel,
        )
        .await
    }
}
