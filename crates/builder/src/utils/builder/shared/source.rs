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
    let had_auth = request.auth.is_some();
    let url = request.url.to_string();
    let mut builder = custom
        .repository()
        .sync_into(work_directory, request.protocol)
        .branch(request.branch)
        .submodules(request.submodules)
        .executor(ctx.executor.clone())
        .context(ctx)
        .cancel_with(cancel);

    if let Some(auth) = request.auth {
        builder = builder.auth(auth);
    }

    builder
        .run()
        .await
        .map_err(|e| explain_clone_error(e, &url, had_auth))
}

/// Turn git's cryptic auth/not-found clone failures into a clear, actionable
/// message. Public-looking HTTPS clones that are actually private (or deleted)
/// make git try to read a username from a non-interactive terminal, yielding
/// "could not read Username ... No such device or address".
fn explain_clone_error(
    error: crate::utils::exec::ExecError,
    url: &str,
    had_auth: bool,
) -> crate::utils::exec::ExecError {
    let text = error.to_string().to_lowercase();
    let looks_like_auth = text.contains("could not read username")
        || text.contains("authentication failed")
        || text.contains("repository not found")
        || text.contains("terminal prompts disabled")
        || (text.contains("fatal") && text.contains("not found"));
    if !looks_like_auth {
        return error;
    }
    let hint = if had_auth {
        "the configured Git credentials don't have access to it"
    } else {
        "it is private or does not exist — connect a Git provider (so OpenOxide \
         can authenticate) or double-check the URL"
    };
    crate::utils::exec::ExecError::CommandFailed {
        code: Some(128),
        stderr: format!("Cannot access repository {url}: {hint}."),
    }
}
