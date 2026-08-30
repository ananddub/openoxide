use std::{convert::Infallible, sync::Arc};

use bytes::Bytes;
use connectrpc::{ConnectRpcBody, ConnectRpcService};
use http_body_util::{BodyExt, Full, combinators::UnsyncBoxBody};
use hyper::{HeaderMap, Request, Response, body::Incoming};
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::conn::auto::Builder,
    service::TowerToHyperService,
};
use tokio::net::TcpListener;
use tower::Service;

type ResponseBody = UnsyncBoxBody<Bytes, Infallible>;

#[derive(Clone)]
struct DispatchService {
    connect: Arc<ConnectRpcService>,
}

impl Service<Request<Incoming>> for DispatchService {
    type Response = Response<ResponseBody>;
    type Error = Infallible;
    type Future = std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>,
    >;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: Request<Incoming>) -> Self::Future {
        let connect = Arc::clone(&self.connect);
        Box::pin(async move { Ok(dispatch(request, connect).await) })
    }
}

pub async fn serve(listener: TcpListener, connect: ConnectRpcService) -> std::io::Result<()> {
    let app = DispatchService {
        connect: Arc::new(connect),
    };

    loop {
        let (stream, peer) = listener.accept().await?;
        let app = app.clone();

        tokio::spawn(async move {
            let io = TokioIo::new(stream);
            let builder = Builder::new(TokioExecutor::new());

            if let Err(error) = builder
                .serve_connection_with_upgrades(io, TowerToHyperService::new(app))
                .await
            {
                tracing::debug!(%peer, %error, "HTTP connection closed with an error");
            }
        });
    }
}

async fn dispatch(
    request: Request<Incoming>,
    connect: Arc<ConnectRpcService>,
) -> Response<ResponseBody> {
    let start = std::time::Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();

    let mut response = if method == "OPTIONS" {
        preflight()
    } else if uri.path().starts_with("/openoxide.api.") || uri.path().starts_with("/grpc.reflection.")
    {
        let mut service = connect.as_ref().clone();
        let outcome = crate::db::reactive::request_scope(service.call(request)).await;
        outcome
            .map(|response| response.map(box_connect_body))
            .unwrap_or_else(|error| match error {})
    } else {
        not_found()
    };

    apply_cors_headers(response.headers_mut());

    let latency = start.elapsed();
    let status = response.status();
    if status.is_server_error() {
        tracing::error!(
            method = %method,
            uri = %uri,
            status = status.as_u16(),
            elapsed = ?latency,
            "HTTP Request failed (SERVER ERROR)"
        );
    } else if status.is_client_error() {
        tracing::warn!(
            method = %method,
            uri = %uri,
            status = status.as_u16(),
            elapsed = ?latency,
            "HTTP Request failed (CLIENT ERROR)"
        );
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
            "HTTP Request completed"
        );
    }

    response
}

fn apply_cors_headers(headers: &mut HeaderMap) {
    use hyper::header::{
        ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
        ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue,
    };

    headers.insert(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
    headers.insert(
        ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("GET, POST, OPTIONS"),
    );
    headers.insert(
        ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static(
            "authorization, content-type, connect-protocol-version, connect-timeout-ms, connect-content-encoding, x-organization-id, x-grpc-web, grpc-timeout, x-user-agent",
        ),
    );
}

fn preflight() -> Response<ResponseBody> {
    let body: Full<Bytes> = Full::default();
    Response::builder()
        .status(204)
        .body(body.map_err(|error| match error {}).boxed_unsync())
        .expect("static preflight response is always valid")
}

fn not_found() -> Response<ResponseBody> {
    let body: Full<Bytes> = Full::new(Bytes::from_static(b"not found"));
    Response::builder()
        .status(404)
        .body(body.map_err(|error| match error {}).boxed_unsync())
        .expect("static not-found response is always valid")
}

fn box_connect_body(body: ConnectRpcBody) -> ResponseBody {
    body.map_err(|error| match error {}).boxed_unsync()
}
