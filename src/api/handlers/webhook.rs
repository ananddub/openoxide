use std::sync::Arc;

use auto_di::singleton;
use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    IncomingWebhookRequest, OperationResult, IncomingWebhookService, IncomingWebhookServiceExt,
};
use crate::services::webhook::{
    DispatchOutcome, GitProviderKind, WebhookCredential, WebhookCredentialStore, WebhookDispatcher,
    WebhookEvent, parse_event, verify_hmac_sha256, verify_token,
};

pub struct WebhookController {
    credentials: Arc<WebhookCredentialStore>,
    dispatcher: Arc<WebhookDispatcher>,
}

#[singleton]
impl WebhookController {
    pub fn new(
        credentials: Arc<WebhookCredentialStore>,
        dispatcher: Arc<WebhookDispatcher>,
    ) -> Self {
        Self {
            credentials,
            dispatcher,
        }
    }
}

fn parse_provider(value: &str) -> Result<GitProviderKind, ConnectError> {
    value
        .parse()
        .map_err(|e: String| ConnectError::not_found(e))
}

fn verify_request(
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

fn supported_event(provider: GitProviderKind, event_name: &str) -> bool {
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

#[auto_grpc::service(connect = true, auth = false)]
impl IncomingWebhookService for WebhookController {
    async fn handle_webhook<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, IncomingWebhookRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let provider = parse_provider(&request.provider)?;
        let provider_id = request.provider_id;

        // Determine event name from the provider + payload metadata.
        // The proto carries the raw payload and a signature; the event
        // name must be inferred from the payload structure.
        let body = request.payload.as_bytes();
        let signature = &request.signature;

        let credential = self
            .credentials
            .load(provider, provider_id)
            .await
            .map_err(|error| ConnectError::unauthenticated(error))?;

        verify_request(provider, &credential, signature, body)?;

        // Extract event name from the payload JSON.
        let event_value: serde_json::Value = serde_json::from_str(&request.payload)
            .map_err(|e| ConnectError::invalid_argument(format!("invalid payload JSON: {e}")))?;

        // For GitHub/Gitea, event name is typically in headers; for ConnectRPC
        // we infer it from the payload structure.
        let event_name = infer_event_name(provider, &event_value);

        if !supported_event(provider, &event_name) {
            return Err(ConnectError::invalid_argument(format!(
                "unsupported webhook event: {event_name}"
            )));
        }

        let event = parse_event(provider, &event_name, body)
            .map_err(|error| ConnectError::invalid_argument(error))?;

        let outcome = match event {
            WebhookEvent::Ping => DispatchOutcome {
                event_type: "ping".into(),
                provider: Some(provider.as_str().into()),
                ..Default::default()
            },
            WebhookEvent::GitRef(ev) => self.dispatcher.dispatch(&ev).await.map_err(|error| {
                ConnectError::new(ErrorCode::Internal, error)
            })?,
            WebhookEvent::PullRequest(ev) => {
                self.dispatcher.dispatch_pull_request(&ev).await.map_err(|error| {
                    ConnectError::new(ErrorCode::Internal, error)
                })?
            }
        };

        let payload = serde_json::to_string(&outcome).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "handle_webhook".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }
}

/// Infer the webhook event name from the JSON payload structure.
fn infer_event_name(provider: GitProviderKind, value: &serde_json::Value) -> String {
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
