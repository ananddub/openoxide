use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    IncomingWebhookRequest, OperationResult, IncomingWebhookService, IncomingWebhookServiceExt,
};
use crate::services::integrations::git::webhooks::{DispatchOutcome, WebhookEvent, parse_event};

use super::helper::*;
use super::init::WebhookController;

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
