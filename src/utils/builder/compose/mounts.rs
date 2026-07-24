use super::compose::ComposeBuilder;
use crate::utils::builder::shared::mounts::prepare_file_mounts;
use crate::utils::{builder::compose::spec::ComposeSpec, exec::ExecResult};
use tokio_util::sync::CancellationToken;

impl ComposeBuilder {
    pub(super) async fn prepare_runtime_files(
        &self,
        spec: &ComposeSpec,
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        if let Some(parent) = std::path::Path::new(&spec.env_file).parent() {
            self.ctx
                .executor
                .run_cancelled("mkdir", ["-p", parent.to_string_lossy().as_ref()], cancel)
                .await?;
        }
        self.ctx
            .write_file_cancelled(&spec.env_file, env_file(spec).as_bytes(), cancel)
            .await?;
        prepare_file_mounts(&self.ctx, &spec.mounts, cancel).await?;
        Ok(())
    }
}

fn env_file(spec: &ComposeSpec) -> String {
    spec.environment
        .iter()
        .map(|(key, value)| format!("{key}={value}\n"))
        .collect()
}
