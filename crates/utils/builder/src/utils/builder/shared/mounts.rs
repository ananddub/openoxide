use crate::utils::{
    builder::{
        shared::BuilderContext,
        spec::{MountKind, MountSpec},
    },
    exec::{ExecError, ExecResult},
};
use tokio_util::sync::CancellationToken;

pub async fn prepare_file_mounts(
    ctx: &BuilderContext,
    mounts: &[MountSpec],
    cancel: &CancellationToken,
) -> ExecResult<()> {
    for mount in mounts {
        if matches!(mount.kind, MountKind::File) {
            let parent = std::path::Path::new(&mount.source)
                .parent()
                .ok_or_else(|| ExecError::CommandFailed {
                    code: None,
                    stderr: "invalid file mount source".into(),
                })?;
            ctx.executor
                .run_cancelled("mkdir", ["-p", parent.to_string_lossy().as_ref()], cancel)
                .await?;
            let content = mount
                .content
                .as_deref()
                .ok_or_else(|| ExecError::CommandFailed {
                    code: None,
                    stderr: format!("file mount {} has no content", mount.target),
                })?;
            ctx.write_file_cancelled(&mount.source, content.as_bytes(), cancel)
                .await?;
        }
    }
    Ok(())
}
