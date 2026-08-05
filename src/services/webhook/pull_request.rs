use serde_json::Value;

use super::types::{GitProviderKind, PullRequestEvent};

pub(super) fn parse(
    provider: GitProviderKind,
    event_name: &str,
    body: &[u8],
) -> Result<PullRequestEvent, String> {
    let value: Value = serde_json::from_slice(body)
        .map_err(|error| format!("invalid pull request payload: {error}"))?;
    match provider {
        GitProviderKind::Github | GitProviderKind::Gitea => github_like(provider, &value),
        GitProviderKind::Gitlab => gitlab(&value),
        GitProviderKind::Bitbucket => bitbucket(event_name, &value),
    }
}

fn github_like(provider: GitProviderKind, value: &Value) -> Result<PullRequestEvent, String> {
    let pull = value
        .get("pull_request")
        .ok_or("pull request data is missing")?;
    Ok(PullRequestEvent {
        provider,
        owner: text(value, "/repository/owner/login")
            .or_else(|| text(value, "/repository/owner/username"))
            .ok_or("repository owner is missing")?,
        repository: text(value, "/repository/name").ok_or("repository name is missing")?,
        number: value
            .get("number")
            .and_then(Value::as_i64)
            .map(|number| number.to_string())
            .ok_or("pull request number is missing")?,
        action: text(value, "/action").unwrap_or_else(|| "updated".into()),
        source_branch: text(pull, "/head/ref").ok_or("source branch is missing")?,
        source_owner: text(pull, "/head/repo/owner/login")
            .or_else(|| text(pull, "/head/repo/owner/username")),
        source_repository: text(pull, "/head/repo/name"),
        target_branch: text(pull, "/base/ref").ok_or("target branch is missing")?,
        commit: text(pull, "/head/sha"),
        author: text(pull, "/user/login").or_else(|| text(pull, "/user/username")),
    })
}

fn gitlab(value: &Value) -> Result<PullRequestEvent, String> {
    let attrs = value
        .get("object_attributes")
        .ok_or("merge request attributes are missing")?;
    Ok(PullRequestEvent {
        provider: GitProviderKind::Gitlab,
        owner: text(value, "/project/path_with_namespace")
            .and_then(|name| name.rsplit_once('/').map(|(owner, _)| owner.to_owned()))
            .ok_or("project namespace is missing")?,
        repository: text(value, "/project/path").ok_or("project path is missing")?,
        number: attrs
            .get("iid")
            .and_then(Value::as_i64)
            .map(|number| number.to_string())
            .ok_or("merge request number is missing")?,
        action: text(attrs, "/action")
            .or_else(|| text(attrs, "/state"))
            .unwrap_or_else(|| "update".into()),
        source_branch: text(attrs, "/source_branch").ok_or("source branch is missing")?,
        source_owner: text(value, "/source/path_with_namespace")
            .and_then(|name| name.rsplit_once('/').map(|(owner, _)| owner.to_owned())),
        source_repository: text(value, "/source/path"),
        target_branch: text(attrs, "/target_branch").ok_or("target branch is missing")?,
        commit: text(attrs, "/last_commit/id"),
        author: text(value, "/user/username"),
    })
}

fn bitbucket(event_name: &str, value: &Value) -> Result<PullRequestEvent, String> {
    let pull = value
        .get("pullrequest")
        .ok_or("pull request data is missing")?;
    Ok(PullRequestEvent {
        provider: GitProviderKind::Bitbucket,
        owner: text(value, "/repository/owner/username")
            .or_else(|| text(value, "/repository/owner/nickname"))
            .ok_or("repository owner is missing")?,
        repository: text(value, "/repository/name").ok_or("repository name is missing")?,
        number: pull
            .get("id")
            .and_then(Value::as_i64)
            .map(|number| number.to_string())
            .ok_or("pull request number is missing")?,
        action: event_name
            .strip_prefix("pullrequest:")
            .unwrap_or("updated")
            .to_owned(),
        source_branch: text(pull, "/source/branch/name").ok_or("source branch is missing")?,
        source_owner: text(pull, "/source/repository/full_name")
            .and_then(|name| name.rsplit_once('/').map(|(owner, _)| owner.to_owned())),
        source_repository: text(pull, "/source/repository/name"),
        target_branch: text(pull, "/destination/branch/name").ok_or("target branch is missing")?,
        commit: text(pull, "/source/commit/hash"),
        author: text(pull, "/author/username").or_else(|| text(pull, "/author/nickname")),
    })
}

fn text(value: &Value, pointer: &str) -> Option<String> {
    value.pointer(pointer)?.as_str().map(str::to_owned)
}
