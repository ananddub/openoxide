use super::application::ApplicationBuilder;
use crate::utils::builder::shared::mounts::prepare_file_mounts;
use crate::utils::{builder::spec::ApplicationSpec, exec::ExecResult};
use tokio_util::sync::CancellationToken;

impl ApplicationBuilder {
    pub(super) async fn prepare_file_mounts(
        &self,
        spec: &ApplicationSpec,
        cancel: &CancellationToken,
    ) -> ExecResult<()> {
        prepare_file_mounts(&self.ctx, &spec.mounts, cancel).await
    }
}
