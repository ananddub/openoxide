use connectrpc::ConnectError;

use crate::services::webhook::{
    GitProviderKind, WebhookCredential, verify_hmac_sha256, verify_token,
};

pub fn parse_provider(value: &str) -> Result<GitProviderKind, ConnectError> {
    value
        .parse()
        .map_err(|e: String| ConnectError::not_found(e))
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
            } else if value.get("ref").is_some() {
                "push hook".into()
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
