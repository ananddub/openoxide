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
    project: Project,
    #[serde(default)]
    commits: Vec<ChangedCommit>,
}

#[derive(Deserialize)]
struct Project {
    path: String,
    path_with_namespace: String,
}

pub(super) fn parse(body: &[u8]) -> Result<PushEvent, String> {
    let payload: Payload = serde_json::from_slice(body)
        .map_err(|error| format!("invalid GitLab push payload: {error}"))?;
    let owner = payload
        .project
        .path_with_namespace
        .rsplit_once('/')
        .map(|(owner, _)| owner.to_owned())
        .ok_or_else(|| "GitLab payload has no project namespace".to_string())?;

    Ok(PushEvent {
        provider: GitProviderKind::Gitlab,
        trigger: if payload.git_ref.starts_with("refs/tags/") {
            GitTrigger::Tag
        } else {
            GitTrigger::Push
        },
        owner,
        repository: payload.project.path,
        branch: branch_from_ref(&payload.git_ref)?,
        before: payload.before,
        after: payload.after,
        changed_paths: collect_paths(payload.commits),
    })
}
