use std::path::{Component, Path};

use tokio_util::sync::CancellationToken;

use crate::utils::{
    builder::{shared::BuilderContext, spec::PatchSpec},
    exec::{ExecError, ExecResult},
};

pub async fn apply_file_patches(
    ctx: &BuilderContext,
    work_directory: &str,
    patches: &[PatchSpec],
    cancel: &CancellationToken,
) -> ExecResult<()> {
    for patch in patches {
        ctx.cancelled(cancel)?;
        let relative = Path::new(&patch.file_path);
        if relative.is_absolute()
            || relative.components().any(|part| {
                matches!(
                    part,
                    Component::ParentDir | Component::RootDir | Component::Prefix(_)
                )
            })
        {
            return Err(ExecError::CommandFailed {
                code: None,
                stderr: format!("unsafe patch path: {}", patch.file_path),
            });
        }

        let target = Path::new(work_directory).join(relative);
        let target = target.to_string_lossy().into_owned();
        if patch.patch_type == "DELETE" {
            ctx.executor
                .run_cancelled("rm", ["-f", target.as_str()], cancel)
                .await?;
            continue;
        }

        let parent = Path::new(&target)
            .parent()
            .ok_or_else(|| ExecError::CommandFailed {
                code: None,
                stderr: format!("invalid patch path: {}", patch.file_path),
            })?;
        ctx.executor
            .run_cancelled("mkdir", ["-p", parent.to_string_lossy().as_ref()], cancel)
            .await?;
        ctx.write_file_cancelled(&target, patch.content.as_bytes(), cancel)
            .await?;
    }
    Ok(())
}
