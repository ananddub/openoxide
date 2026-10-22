use connectrpc::ConnectError;

use crate::api::connect::generated::openoxide::api::v1::{
    IncomingWebhookDispatchOutcome as IncomingWebhookDispatchOutcomeProto,
    WebhookProviderType as ProtoWebhookProviderType,
};
use crate::services::integrations::git::webhooks::{
    DispatchOutcome, GitProviderKind, WebhookCredential, verify_hmac_sha256, verify_token,
};

pub fn parse_provider(
    value: ::buffa::EnumValue<ProtoWebhookProviderType>,
) -> Result<GitProviderKind, ConnectError> {
    match value {
        ::buffa::EnumValue::Known(ProtoWebhookProviderType::Github) => Ok(GitProviderKind::Github),
        ::buffa::EnumValue::Known(ProtoWebhookProviderType::Gitlab) => Ok(GitProviderKind::Gitlab),
        ::buffa::EnumValue::Known(ProtoWebhookProviderType::Gitea) => Ok(GitProviderKind::Gitea),
        ::buffa::EnumValue::Known(ProtoWebhookProviderType::Bitbucket) => {
            Ok(GitProviderKind::Bitbucket)
        }
        _ => Err(ConnectError::invalid_argument(
            "unsupported webhook provider",
        )),
    }
}

pub fn outcome_to_proto(outcome: DispatchOutcome) -> IncomingWebhookDispatchOutcomeProto {
    IncomingWebhookDispatchOutcomeProto {
        event_type: outcome.event_type,
        provider: outcome.provider,
        owner: outcome.owner,
        repository: outcome.repository,
        applications_queued: outcome.applications_queued as i64,
        compose_projects_queued: outcome.compose_projects_queued as i64,
        skipped_by_watch_paths: outcome.skipped_by_watch_paths as i64,
        already_running: outcome.already_running as i64,
        pull_request_number: outcome.pull_request_number,
        pull_request_action: outcome.pull_request_action,
        source_branch: outcome.source_branch,
        source_owner: outcome.source_owner,
        source_repository: outcome.source_repository,
        target_branch: outcome.target_branch,
        commit: outcome.commit,
        author: outcome.author,
        previews_created: outcome.previews_created as i64,
        previews_redeployed: outcome.previews_redeployed as i64,
        previews_removed: outcome.previews_removed as i64,
        previews_skipped_permission: outcome.previews_skipped_permission as i64,
        previews_skipped_limit: outcome.previews_skipped_limit as i64,
    }
}

pub fn verify_request(
    provider: GitProviderKind,
    credential: &WebhookCredential,
    signature: &str,
    body: &[u8],
) -> Result<(), ConnectError> {
    let valid = match (provider, credential) {
        (GitProviderKind::Github, WebhookCredential::Hmac(secret)) => {
            verify_hmac_sha256(secret.as_bytes(), body, signature)
        }
        (GitProviderKind::Gitea, WebhookCredential::Hmac(secret)) => {
            verify_hmac_sha256(secret.as_bytes(), body, signature)
        }
        (GitProviderKind::Gitlab, WebhookCredential::Token(secret)) => {
            verify_token(secret, Some(signature))
        }
        (GitProviderKind::Bitbucket, WebhookCredential::Hmac(secret)) => {
            verify_hmac_sha256(secret.as_bytes(), body, signature)
        }
        _ => false,
    };
    valid
        .then_some(())
        .ok_or_else(|| ConnectError::unauthenticated("invalid webhook signature or token"))
}

pub fn supported_event(provider: GitProviderKind, event_name: &str) -> bool {
    let normalized = event_name.to_ascii_lowercase();
    match provider {
        GitProviderKind::Github | GitProviderKind::Gitea => {
            matches!(
                normalized.as_str(),
                "push" | "create" | "pull_request" | "ping"
            )
        }
        GitProviderKind::Gitlab => matches!(
            normalized.as_str(),
            "push hook" | "tag push hook" | "merge request hook"
        ),
        GitProviderKind::Bitbucket => {
            normalized == "repo:push"
                || normalized == "repo:refs_changed"
                || normalized == "diagnostics:repo:ping"
                || normalized.starts_with("pullrequest:")
        }
    }
}

/// Infer the webhook event name from the JSON payload structure.
pub fn infer_event_name(provider: GitProviderKind, value: &serde_json::Value) -> String {
    match provider {
        GitProviderKind::Github => {
            if value.get("pull_request").is_some() {
                "pull_request".into()
            } else if value.get("ref_type").is_some() {
                "create".into()
            } else {
                "push".into()
            }
        }
        GitProviderKind::Gitlab => {
            if value.get("object_kind").and_then(|v| v.as_str()) == Some("merge_request") {
                "merge request hook".into()
            } else {
                "push hook".into()
            }
        }
        GitProviderKind::Gitea => {
            if value.get("pull_request").is_some() {
                "pull_request".into()
            } else {
                "push".into()
            }
        }
        GitProviderKind::Bitbucket => {
            if value.get("pullrequest").is_some() {
                "pullrequest:updated".into()
            } else {
                "repo:push".into()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_invalid_github_signature() {
        let result = verify_request(
            GitProviderKind::Github,
            &WebhookCredential::Hmac("secret".into()),
            "sha256=deadbeef",
            b"payload",
        );
        assert!(result.is_err());
    }

    #[test]
    fn accepts_supported_event_headers() {
        assert!(supported_event(GitProviderKind::Gitlab, "Tag Push Hook"));
    }

    #[test]
    fn rejects_unknown_event_headers() {
        assert!(!supported_event(GitProviderKind::Github, "issues"));
    }
}
