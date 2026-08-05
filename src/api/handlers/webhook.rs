use auto_route::controller;
use axum::{
    Json,
    body::Bytes,
    extract::Path,
    http::{HeaderMap, StatusCode},
};
use std::sync::Arc;

use crate::services::webhook::{
    DispatchOutcome, GitProviderKind, WebhookCredential, WebhookCredentialStore, WebhookDispatcher,
    WebhookEvent, parse_event, verify_hmac_sha256, verify_token,
};

type ApiError = (StatusCode, String);

pub struct WebhookController {
    credentials: Arc<WebhookCredentialStore>,
    dispatcher: Arc<WebhookDispatcher>,
}

#[controller("/public/webhooks")]
impl WebhookController {
    fn new(credentials: Arc<WebhookCredentialStore>, dispatcher: Arc<WebhookDispatcher>) -> Self {
        Self {
            credentials,
            dispatcher,
        }
    }

    #[post("/{provider}/{provider_id}")]
    async fn receive(
        &self,
        Path((provider, provider_id)): Path<(String, i64)>,
        headers: HeaderMap,
        body: Bytes,
    ) -> Result<Json<crate::services::webhook::DispatchOutcome>, ApiError> {
        let provider = parse_provider(&provider)?;
        let event_name = event_header(provider, &headers)?;
        let credential = self
            .credentials
            .load(provider, provider_id)
            .await
            .map_err(|error| (StatusCode::UNAUTHORIZED, error))?;
        verify_request(provider, &credential, &headers, &body)?;
        let event = parse_event(provider, event_name, &body)
            .map_err(|error| (StatusCode::BAD_REQUEST, error))?;
        match event {
            WebhookEvent::Ping => Ok(Json(DispatchOutcome {
                event_type: "ping".into(),
                provider: Some(provider.as_str().into()),
                ..Default::default()
            })),
            WebhookEvent::GitRef(event) => self
                .dispatcher
                .dispatch(&event)
                .await
                .map(Json)
                .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error)),
            WebhookEvent::PullRequest(event) => self
                .dispatcher
                .dispatch_pull_request(&event)
                .await
                .map(Json)
                .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error)),
        }
    }
}

fn parse_provider(value: &str) -> Result<GitProviderKind, ApiError> {
    match value.to_ascii_lowercase().as_str() {
        "github" => Ok(GitProviderKind::Github),
        "gitlab" => Ok(GitProviderKind::Gitlab),
        "gitea" => Ok(GitProviderKind::Gitea),
        "bitbucket" => Ok(GitProviderKind::Bitbucket),
        _ => Err((StatusCode::NOT_FOUND, "unsupported Git provider".into())),
    }
}

fn event_header<'a>(
    provider: GitProviderKind,
    headers: &'a HeaderMap,
) -> Result<&'a str, ApiError> {
    let name = match provider {
        GitProviderKind::Github => "x-github-event",
        GitProviderKind::Gitlab => "x-gitlab-event",
        GitProviderKind::Gitea => "x-gitea-event",
        GitProviderKind::Bitbucket => "x-event-key",
    };
    let value = header(headers, name).ok_or((
        StatusCode::BAD_REQUEST,
        format!("missing webhook event header: {name}"),
    ))?;
    let normalized = value.to_ascii_lowercase();
    let supported = match provider {
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
    };
    supported.then_some(value).ok_or((
        StatusCode::UNPROCESSABLE_ENTITY,
        format!("unsupported webhook event: {value}"),
    ))
}

fn verify_request(
    provider: GitProviderKind,
    credential: &WebhookCredential,
    headers: &HeaderMap,
    body: &[u8],
) -> Result<(), ApiError> {
    let valid = match (provider, credential) {
        (GitProviderKind::Github, WebhookCredential::Hmac(secret)) => {
            header(headers, "x-hub-signature-256")
                .is_some_and(|signature| verify_hmac_sha256(secret.as_bytes(), body, signature))
        }
        (GitProviderKind::Gitea, WebhookCredential::Hmac(secret)) => {
            header(headers, "x-gitea-signature")
                .is_some_and(|signature| verify_hmac_sha256(secret.as_bytes(), body, signature))
        }
        (GitProviderKind::Gitlab, WebhookCredential::Token(secret)) => {
            verify_token(secret, header(headers, "x-gitlab-token"))
        }
        (GitProviderKind::Bitbucket, WebhookCredential::Hmac(secret)) => headers
            .get("x-hub-signature")
            .and_then(|value| value.to_str().ok())
            .is_some_and(|signature| verify_hmac_sha256(secret.as_bytes(), body, signature)),
        _ => false,
    };
    valid.then_some(()).ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            "invalid webhook signature or token".into(),
        )
    })
}

fn header<'a>(headers: &'a HeaderMap, name: &str) -> Option<&'a str> {
    headers.get(name)?.to_str().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_invalid_github_signature() {
        let mut headers = HeaderMap::new();
        headers.insert("x-hub-signature-256", "sha256=deadbeef".parse().unwrap());
        let result = verify_request(
            GitProviderKind::Github,
            &WebhookCredential::Hmac("secret".into()),
            &headers,
            b"payload",
        );
        assert_eq!(result.unwrap_err().0, StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn accepts_supported_event_headers() {
        let mut headers = HeaderMap::new();
        headers.insert("x-gitlab-event", "Tag Push Hook".parse().unwrap());
        assert_eq!(
            event_header(GitProviderKind::Gitlab, &headers).unwrap(),
            "Tag Push Hook"
        );
    }

    #[test]
    fn rejects_unknown_event_headers() {
        let mut headers = HeaderMap::new();
        headers.insert("x-github-event", "issues".parse().unwrap());
        assert_eq!(
            event_header(GitProviderKind::Github, &headers)
                .unwrap_err()
                .0,
            StatusCode::UNPROCESSABLE_ENTITY
        );
    }
}
