use serde::Deserialize;

use super::types::{GitProviderKind, GitTrigger, PushEvent};

#[derive(Deserialize)]
struct Payload {
    repository: Repository,
    push: Push,
}

#[derive(Deserialize)]
struct Repository {
    name: String,
    owner: Owner,
}

#[derive(Deserialize)]
struct Owner {
    username: Option<String>,
    nickname: Option<String>,
    display_name: Option<String>,
}

#[derive(Deserialize)]
struct Push {
    changes: Vec<Change>,
}

#[derive(Deserialize)]
struct Change {
    #[serde(rename = "new")]
    new_ref: Option<Reference>,
    old: Option<Reference>,
}

#[derive(Deserialize)]
struct Reference {
    name: String,
    #[serde(rename = "type")]
    kind: Option<String>,
    target: Option<Target>,
}

#[derive(Deserialize)]
struct Target {
    hash: String,
}

pub(super) fn parse(body: &[u8]) -> Result<PushEvent, String> {
    let payload: Payload = serde_json::from_slice(body)
        .map_err(|error| format!("invalid Bitbucket push payload: {error}"))?;
    let change = payload
        .push
        .changes
        .into_iter()
        .next()
        .ok_or_else(|| "Bitbucket push contains no changes".to_string())?;
    let branch = change
        .new_ref
        .as_ref()
        .or(change.old.as_ref())
        .map(|reference| reference.name.clone())
        .ok_or_else(|| "Bitbucket push contains no branch".to_string())?;
    let owner = payload
        .repository
        .owner
        .username
        .or(payload.repository.owner.nickname)
        .or(payload.repository.owner.display_name)
        .ok_or_else(|| "Bitbucket payload has no repository owner".to_string())?;

    Ok(PushEvent {
        provider: GitProviderKind::Bitbucket,
        trigger: change
            .new_ref
            .as_ref()
            .or(change.old.as_ref())
            .and_then(|reference| reference.kind.as_deref())
            .filter(|kind| *kind == "tag")
            .map(|_| GitTrigger::Tag)
            .unwrap_or(GitTrigger::Push),
        owner,
        repository: payload.repository.name,
        branch,
        before: change
            .old
            .and_then(|value| value.target)
            .map(|target| target.hash),
        after: change
            .new_ref
            .and_then(|value| value.target)
            .map(|target| target.hash),
        changed_paths: Vec::new(),
    })
}
