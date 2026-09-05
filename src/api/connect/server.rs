use std::{convert::Infallible, sync::Arc};

use bytes::Bytes;
use connectrpc::{ConnectRpcBody, ConnectRpcService};
use http_body_util::{BodyExt, Full, combinators::UnsyncBoxBody};
use hyper::{
    HeaderMap, Method, Request, Response, body::Incoming, server::conn::http2,
};
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    service::TowerToHyperService,
};
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use tower::Service;

use crate::api::dashboard;

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

/// Serves the ConnectRPC dispatch over HTTPS/HTTP-2 only. HTTP/1.1 is
/// deliberately disabled: the TLS config advertises h2 alone, so cleartext
/// and h1 clients fail at handshake instead of half-working. Matching the
/// testing_connect_rpc prototype (mkcert dev certs, ALPN ["h2"]).
pub async fn serve(
    listener: TcpListener,
    connect: ConnectRpcService,
    tls: Arc<rustls::ServerConfig>,
) -> std::io::Result<()> {
    let app = DispatchService {
        connect: Arc::new(connect),
    };
    let acceptor = TlsAcceptor::from(tls);

    loop {
        let (stream, peer) = listener.accept().await?;
        let app = app.clone();
        let acceptor = acceptor.clone();

        tokio::spawn(async move {
            let tls_stream = match acceptor.accept(stream).await {
                Ok(tls) => tls,
                Err(error) => {
                    tracing::debug!(%peer, %error, "TLS handshake failed (non-h2 client or probe)");
                    return;
                }
            };

            let io = TokioIo::new(tls_stream);
            // Explicit http2 builder: even post-TLS, HTTP/1.1 is rejected —
            // the connection only speaks h2.
            let builder = http2::Builder::new(TokioExecutor::new());

            if let Err(error) = builder
                .serve_connection(io, TowerToHyperService::new(app))
                .await
            {
                tracing::debug!(%peer, %error, "HTTP/2 connection closed with an error");
            }
        });
    }
}

/// Loads the PEM certificate chain + key and builds the h2-only server
/// config (ALPN advertises HTTP/2 exclusively).
pub fn tls_config(cert_path: &str, key_path: &str) -> std::io::Result<Arc<rustls::ServerConfig>> {
    let cert_pem = std::fs::read(cert_path)?;
    let key_pem = std::fs::read(key_path)?;

    let certs: Vec<CertificateDer<'_>> = rustls_pemfile::certs(&mut cert_pem.as_slice())
        .collect::<Result<_, _>>()
        .map_err(|error| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("invalid TLS certificate chain in {cert_path}: {error}"),
            )
        })?;
    let key: PrivateKeyDer<'_> = rustls_pemfile::private_key(&mut key_pem.as_slice())
        .map_err(|error| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("invalid TLS key in {key_path}: {error}"),
            )
        })?
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("no private key found in {key_path}"),
            )
        })?;

    let mut config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .map_err(|error| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("TLS cert/key mismatch: {error}"),
            )
        })?;

    // ALPN advertises HTTP/2 exclusively: an HTTP/1.1-only client fails the
    // handshake ("no application protocol") instead of half-working.
    config.alpn_protocols = vec![b"h2".to_vec()];

    Ok(Arc::new(config))
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
        dashboard::serve_spa(&method, uri.path())
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
