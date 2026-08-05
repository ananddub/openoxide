use serde::Deserialize;

use super::types::{GitProviderKind, GitTrigger, PushEvent};

#[derive(Deserialize)]
struct Payload {
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
    let value: serde_json::Value =
        serde_json::from_slice(body).map_err(|error| format!("invalid tag payload: {error}"))?;
    if value.get("ref_type").and_then(|value| value.as_str()) != Some("tag") {
        return Err("create event is not a tag".into());
    }
    let payload: Payload =
        serde_json::from_value(value).map_err(|error| format!("invalid tag payload: {error}"))?;
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
