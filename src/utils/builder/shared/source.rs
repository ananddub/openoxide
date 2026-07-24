use crate::utils::{builder::shared::BuilderContext, exec::ExecResult, provider::CustomClient};
use tokio_util::sync::CancellationToken;

pub async fn fetch_git_repository(
    ctx: &BuilderContext,
    work_directory: &str,
    url: &str,
    branch: &str,
    submodules: bool,
    protocol: crate::utils::provider::CloneProtocol,
    auth: Option<crate::utils::git::types::GitAuth>,
    cancel: &CancellationToken,
) -> ExecResult<()> {
    let custom = CustomClient::new(url);
    let mut builder = custom
        .repository()
        .sync_into(work_directory, protocol)
        .branch(branch)
        .submodules(submodules)
        .context(ctx)
        .cancel_with(cancel);

    if let Some(auth) = auth {
        builder = builder.auth(auth);
    }

    builder.run().await
}
