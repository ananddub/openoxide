use serde::Deserialize;

use super::types::{GitProviderKind, GitTrigger, PushEvent};

#[derive(Deserialize)]
struct Payload {
    ref_type: String,
    #[serde(rename = "ref")]
    reference: String,
    repository: Repository,
    after: Option<String>,
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

pub(super) fn parse_create(provider: GitProviderKind, body: &[u8]) -> Result<PushEvent, String> {
    let payload: Payload =
        serde_json::from_slice(body).map_err(|error| format!("invalid tag payload: {error}"))?;
    if payload.ref_type != "tag" {
        return Err("create event is not a tag".into());
    }
    let owner = payload
        .repository
        .owner
        .login
        .or(payload.repository.owner.username)
        .or(payload.repository.owner.name)
        .ok_or("tag payload has no repository owner")?;
    Ok(PushEvent {
        provider,
        trigger: GitTrigger::Tag,
        owner,
        repository: payload.repository.name,
        branch: payload.reference,
        before: None,
        after: payload.after,
        changed_paths: Vec::new(),
    })
}
