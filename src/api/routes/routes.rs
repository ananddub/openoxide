use std::sync::Arc;
use std::time::Instant;

use auto_di::singleton;
use axum::{
    Router,
    extract::Request,
    middleware::{self, Next},
    response::Response,
};
use tower_http::cors::{Any, CorsLayer};

use crate::api::routes::socket::Socket;

async fn request_duration_middleware(req: Request, next: Next) -> Response {
    let start = Instant::now();
    let method = req.method().clone();
    let uri = req.uri().clone();

    let response = next.run(req).await;

    let latency = start.elapsed();
    let status = response.status();

    if status.is_server_error() || latency.as_millis() >= 1000 {
        tracing::warn!(
            method = %method,
            uri = %uri,
            status = status.as_u16(),
            elapsed = ?latency,
            "HTTP Request completed (slow/error)"
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
        .layer(middleware::from_fn(request_duration_middleware))
        .layer(sock.layer.clone())
        .layer(cors)
}
