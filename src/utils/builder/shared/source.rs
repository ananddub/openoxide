use crate::utils::{builder::shared::BuilderContext, exec::ExecResult, provider::CustomClient};
use tokio_util::sync::CancellationToken;

/// Everything a single `git clone` needs, so the fetch entry point stays small.
#[derive(Clone, Debug)]
pub struct GitRequest<'a> {
    pub url: &'a str,
    pub branch: &'a str,
    pub submodules: bool,
    pub protocol: crate::utils::provider::CloneProtocol,
    pub auth: Option<crate::utils::git::types::GitAuth>,
}

pub async fn fetch_git_repository(
    ctx: &BuilderContext,
    work_directory: &str,
    request: GitRequest<'_>,
    cancel: &CancellationToken,
) -> ExecResult<()> {
    let custom = CustomClient::new(request.url);
    let mut builder = custom
        .repository()
        .sync_into(work_directory, request.protocol)
        .branch(request.branch)
        .submodules(request.submodules)
        .context(ctx)
        .cancel_with(cancel);

    if let Some(auth) = request.auth {
        builder = builder.auth(auth);
    }

    builder.run().await
}
