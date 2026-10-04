use super::{builder::ComposeBuilder, spec::ComposeSource};
use crate::utils::builder::shared::source::{GitRequest, fetch_git_repository};
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
                fetch_git_repository(
                    &self.ctx,
                    &spec.work_directory,
                    GitRequest {
                        url: url.as_str(),
                        branch: branch.as_str(),
                        submodules: *submodules,
                        protocol: *protocol,
                        auth: auth.clone(),
                    },
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
}
