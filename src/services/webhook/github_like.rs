use serde::Deserialize;

use super::{
    shared::{ChangedCommit, branch_from_ref, collect_paths},
    types::{GitProviderKind, GitTrigger, PushEvent},
};

#[derive(Deserialize)]
struct Payload {
    #[serde(rename = "ref")]
    git_ref: String,
    before: Option<String>,
    after: Option<String>,
    repository: Repository,
    #[serde(default)]
    commits: Vec<ChangedCommit>,
}

#[derive(Deserialize)]
struct Repository {
    name: String,
    owner: Owner,
}

#[derive(Deserialize)]
struct Owner {
    login: Option<String>,
    username: Option<String>,
    name: Option<String>,
}

pub(super) fn parse(provider: GitProviderKind, body: &[u8]) -> Result<PushEvent, String> {
    let payload: Payload = serde_json::from_slice(body)
        .map_err(|error| format!("invalid GitHub/Gitea push payload: {error}"))?;
    let owner = payload
        .repository
        .owner
        .login
        .or(payload.repository.owner.username)
        .or(payload.repository.owner.name)
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| "push payload has no repository owner".to_string())?;

    Ok(PushEvent {
        provider,
        trigger: if payload.git_ref.starts_with("refs/tags/") {
            GitTrigger::Tag
        } else {
            GitTrigger::Push
        },
        owner,
        repository: payload.repository.name,
        branch: branch_from_ref(&payload.git_ref)?,
        before: payload.before,
        after: payload.after,
        changed_paths: collect_paths(payload.commits),
    })
}
