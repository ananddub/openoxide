mod auth;
mod bitbucket;
mod credentials;
mod dispatcher;
mod github_like;
mod gitlab;
mod pull_request;
mod shared;
mod tag;
mod types;

pub use auth::{sign_hmac_sha256, verify_hmac_sha256, verify_token};
pub use credentials::{WebhookCredential, WebhookCredentialStore};
pub use dispatcher::{DispatchOutcome, WebhookDispatcher};
pub use types::{GitProviderKind, GitTrigger, PullRequestEvent, PushEvent, WebhookEvent};

pub fn parse_push(provider: GitProviderKind, body: &[u8]) -> Result<PushEvent, String> {
    match provider {
        GitProviderKind::Github | GitProviderKind::Gitea => github_like::parse(provider, body),
        GitProviderKind::Gitlab => gitlab::parse(body),
        GitProviderKind::Bitbucket => bitbucket::parse(body),
    }
}

pub fn parse_event(
    provider: GitProviderKind,
    event_name: &str,
    body: &[u8],
) -> Result<WebhookEvent, String> {
    let event_name = event_name.to_ascii_lowercase();
    if matches!(event_name.as_str(), "ping" | "diagnostics:repo:ping") {
        return Ok(WebhookEvent::Ping);
    }
    let is_pull_request = match provider {
        GitProviderKind::Github | GitProviderKind::Gitea => event_name == "pull_request",
        GitProviderKind::Gitlab => event_name == "merge request hook",
        GitProviderKind::Bitbucket => event_name.starts_with("pullrequest:"),
    };
    if is_pull_request {
        return pull_request::parse(provider, &event_name, body).map(WebhookEvent::PullRequest);
    }
    if matches!(provider, GitProviderKind::Github | GitProviderKind::Gitea)
        && event_name == "create"
    {
        return tag::parse_create(provider, body).map(WebhookEvent::GitRef);
    }
    parse_push(provider, body).map(WebhookEvent::GitRef)
}

#[cfg(test)]
mod tests;
