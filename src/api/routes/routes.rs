use std::sync::Arc;
use std::time::Instant;

use auto_di::singleton;
use axum::{
    Router,
    body::{Body, HttpBody, to_bytes},
    extract::Path,
    extract::Request,
    http::header::CONTENT_TYPE,
    middleware::{self, Next},
    response::sse::{Event, Sse},
    response::{IntoResponse, Response},
};
use tower_http::cors::{Any, CorsLayer};

use crate::api::routes::socket::Socket;

async fn request_duration_middleware(req: Request, next: Next) -> Response {
    let start = Instant::now();
    let method = req.method().clone();
    let uri = req.uri().clone();

    let response = crate::db::reactive::request_scope(next.run(req)).await;

    let latency = start.elapsed();
    let status = response.status();

    if status.is_client_error() || status.is_server_error() {
        let (response, error_detail) = error_response_detail(response).await;
        if status.is_server_error() {
            tracing::error!(
                method = %method,
                uri = %uri,
                status = status.as_u16(),
                elapsed = ?latency,
                error = %error_detail,
                "HTTP Request failed (SERVER ERROR)"
            );
        } else {
            tracing::warn!(
                method = %method,
                uri = %uri,
                status = status.as_u16(),
                elapsed = ?latency,
                error = %error_detail,
                "HTTP Request failed (CLIENT ERROR)"
            );
        }
        return response;
    } else if latency.as_millis() >= 1000 {
        tracing::warn!(
            method = %method,
            uri = %uri,
            status = status.as_u16(),
            elapsed = ?latency,
            "HTTP Request completed (SLOW)"
        );
    } else {
        tracing::info!(
            method = %method,
            uri = %uri,
            status = status.as_u16(),
            elapsed = ?latency,
            "HTTP Request completed"
        );
    }

    response
}

const MAX_LOGGED_ERROR_BODY: usize = 64 * 1024;

async fn error_response_detail(response: Response) -> (Response, String) {
    let content_type = response
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_owned();
    let size = response.body().size_hint().upper();
    if size.is_none_or(|size| size > MAX_LOGGED_ERROR_BODY as u64) {
        return (
            response,
            "response body unavailable or too large".to_owned(),
        );
    }

    let (parts, body) = response.into_parts();
    match to_bytes(body, MAX_LOGGED_ERROR_BODY).await {
        Ok(bytes) => {
            let detail = format_error_body(&bytes, &content_type);
            (Response::from_parts(parts, Body::from(bytes)), detail)
        }
        Err(error) => (
            Response::from_parts(parts, Body::empty()),
            format!("failed to read error response body: {error}"),
        ),
    }
}

fn format_error_body(bytes: &[u8], content_type: &str) -> String {
    if bytes.is_empty() {
        return "empty response body".to_owned();
    }
    if content_type.contains("json") {
        if let Ok(mut value) = serde_json::from_slice::<serde_json::Value>(bytes) {
            redact_sensitive_json(&mut value);
            return value.to_string();
        }
    }
    String::from_utf8_lossy(bytes).into_owned()
}

fn redact_sensitive_json(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(items) => {
            for (key, value) in items {
                let key = key.to_ascii_lowercase();
                if [
                    "password",
                    "token",
                    "secret",
                    "authorization",
                    "private_key",
                ]
                .iter()
                .any(|name| key.contains(name))
                {
                    *value = serde_json::Value::String("[REDACTED]".to_owned());
                } else {
                    redact_sensitive_json(value);
                }
            }
        }
        serde_json::Value::Array(items) => {
            for value in items {
                redact_sensitive_json(value);
            }
        }
        _ => {}
    }
}

#[singleton]
pub async fn router_init(sock: Arc<Socket>) -> Router<()> {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    auto_route::routes()
        .await
        .expect("failed to build auto-registered controller routes")
        .merge(auto_route::openapi_routes("/openapi.json", "/swagger-ui"))
        .merge(auto_route::scalar_routes("/scalar", "/openapi.json"))
        .route(
            "/_openoxide/html/events/{session}",
            axum::routing::get(html_events),
        )
        .layer(middleware::from_fn(request_duration_middleware))
        .layer(sock.layer.clone())
        .layer(cors)
}

async fn html_events(Path(session): Path<String>) -> Response {
    let Some(receiver) = html_rt::take_session(&session) else {
        return axum::http::StatusCode::NOT_FOUND.into_response();
    };
    let stream = futures::stream::unfold(receiver, |mut receiver| async move {
        receiver.recv().await.map(|patch| {
            let payload = serde_json::json!({"slot": patch.slot, "html": patch.html});
            (
                Ok::<Event, std::convert::Infallible>(Event::default().data(payload.to_string())),
                receiver,
            )
        })
    });
    Sse::new(stream).into_response()
}
