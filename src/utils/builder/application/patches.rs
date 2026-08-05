use tokio_util::sync::CancellationToken;

use crate::utils::{
    builder::{
        application::application::ApplicationBuilder,
        spec::{ApplicationSpec, SourceSpec},
    },
    exec::{ExecError, ExecResult},
};

impl ApplicationBuilder {
    pub(super) async fn apply_patches(
        &self,
        spec: &ApplicationSpec,
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        if spec.patches.is_empty() {
            return Ok(());
        }
        if matches!(spec.source, SourceSpec::Docker { .. }) {
            return Err(ExecError::CommandFailed {
                code: None,
                stderr: "file patches require a repository-based application source".into(),
            });
        }

        crate::utils::builder::shared::patches::apply_file_patches(
            &self.ctx,
            &spec.work_directory,
            &spec.patches,
            cancel,
        )
        .await
    }
}
