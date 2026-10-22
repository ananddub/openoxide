use std::{convert::Infallible, sync::Arc};

use bytes::Bytes;
use connectrpc::{ConnectRpcBody, ConnectRpcService};
use http_body_util::{BodyExt, Full, combinators::UnsyncBoxBody};
use hyper::{HeaderMap, Request, Response, body::Incoming, server::conn::http2};
use hyper_util::{
    rt::{TokioExecutor, TokioIo, TokioTimer},
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
            let mut builder = http2::Builder::new(TokioExecutor::new());
            // Keepalive PINGs so long-running unary calls (the AI assistant's
            // multi-step tool loop can take 30-60s with no bytes flowing until
            // the final frame) don't look idle and get dropped by the client /
            // OS as a dead connection — the symptom was a browser "Failed to
            // fetch" mid-chat.
            builder
                .timer(TokioTimer::new())
                .keep_alive_interval(std::time::Duration::from_secs(20))
                .keep_alive_timeout(std::time::Duration::from_secs(60));

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
    } else if uri.path().starts_with("/openoxide.api.")
        || uri.path().starts_with("/grpc.reflection.")
    {
        let mut service = connect.as_ref().clone();
        let outcome = service.call(request).await;
        outcome
            .map(|response| response.map(box_connect_body))
            .unwrap_or_else(|error| match error {})
    } else if uri.path() == "/mcp" {
        mcp_response(request).await
    } else if uri.path() == "/git-providers/github/manifest/callback" {
        // GitHub App manifest flow: the browser lands here with ?code= after
        // creating the app on GitHub — exchange it, store the provider, and
        // send the user back to the git providers page.
        github_manifest_callback(&uri).await
    } else if uri.path().starts_with("/api/") {
        // The legacy REST surface was fully retired: every former `/api/*`
        // consumer now streams through ConnectRPC. 404 explicitly so stale
        // clients fail fast instead of silently getting the SPA shell.
        let body: Full<Bytes> = Full::new(Bytes::from_static(
            br#"{"code":"not_found","message":"legacy REST API removed; use the ConnectRPC API"}"#,
        ));
        return Response::builder()
            .status(404)
            .header("content-type", "application/json")
            .body(body.map_err(|error| match error {}).boxed_unsync())
            .expect("static 404 response is always valid");
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

/// MCP Streamable-HTTP endpoint: a single JSON-RPC request in, a JSON-RPC
/// response (or 204 for notifications) out. The caller's `Authorization` +
/// `x-organization-id` headers are forwarded so each `tools/call` runs under
/// their identity through the same in-process dispatch the assistant uses.
async fn mcp_response(request: Request<Incoming>) -> Response<ResponseBody> {
    let (parts, body) = request.into_parts();
    let authorization = parts
        .headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);
    let organization = parts
        .headers
        .get("x-organization-id")
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);

    let bytes = match body.collect().await {
        Ok(c) => c.to_bytes(),
        Err(_) => {
            return json_response(
                400,
                br#"{"jsonrpc":"2.0","id":null,"error":{"code":-32600,"message":"could not read request body"}}"#
                    .to_vec(),
            );
        }
    };

    let result =
        crate::api::mcp::handle_jsonrpc(&bytes, authorization.as_deref(), organization.as_deref())
            .await;

    match result {
        // Notifications produce no body.
        None => Response::builder()
            .status(202)
            .body(
                Full::<Bytes>::default()
                    .map_err(|e| match e {})
                    .boxed_unsync(),
            )
            .expect("static 202 response is always valid"),
        Some(value) => {
            let payload = serde_json::to_vec(&value)
                .unwrap_or_else(|_| b"{\"jsonrpc\":\"2.0\",\"id\":null,\"error\":{\"code\":-32603,\"message\":\"encode error\"}}".to_vec());
            json_response(200, payload)
        }
    }
}

fn json_response(status: u16, body: Vec<u8>) -> Response<ResponseBody> {
    Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(
            Full::new(Bytes::from(body))
                .map_err(|e| match e {})
                .boxed_unsync(),
        )
        .expect("json response is always valid")
}

fn apply_cors_headers(headers: &mut HeaderMap) {
    use hyper::header::{
        ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue,
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

fn box_connect_body(body: ConnectRpcBody) -> ResponseBody {
    body.map_err(|error| match error {}).boxed_unsync()
}

/// GitHub App manifest callback: `POST {api}/app-manifests/{code}/conversions`
/// turns the temporary code into the app's credentials; we persist them as a
/// GITHUB git provider and redirect the browser back to the providers page.
async fn github_manifest_callback(uri: &hyper::Uri) -> Response<ResponseBody> {
    use crate::services::integrations::git::{
        CreateProvider, GitProviderService, ProviderCredentials,
    };

    let fail = |message: &str| -> Response<ResponseBody> {
        json_response(
            400,
            format!(
                "{{\"code\":\"github_manifest\",\"message\":{}}}",
                serde_json::to_string(message).unwrap_or_default()
            )
            .into_bytes(),
        )
    };

    let query: std::collections::HashMap<String, String> =
        url::form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes())
            .map(|(key, value)| (key.into_owned(), value.into_owned()))
            .collect();

    let Some(code) = query.get("code").filter(|value| !value.is_empty()) else {
        return fail("missing manifest code");
    };
    let Some(organization_id) = query
        .get("organization_id")
        .and_then(|value| value.parse::<i64>().ok())
    else {
        return fail("missing organization_id");
    };
    let github_base = query
        .get("gh")
        .map(|value| value.trim_end_matches('/').to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "https://github.com".to_string());
    let api_base = if github_base == "https://github.com" {
        "https://api.github.com".to_string()
    } else {
        format!("{github_base}/api/v3")
    };

    #[derive(serde::Deserialize)]
    struct Conversion {
        id: i64,
        slug: Option<String>,
        name: Option<String>,
        client_id: Option<String>,
        client_secret: Option<String>,
        // GitHub's webhook_secret is ignored — the panel issues its own per-provider secret.
        pem: Option<String>,
    }

    let response = match reqwest::Client::new()
        .post(format!("{api_base}/app-manifests/{code}/conversions"))
        .header("accept", "application/vnd.github+json")
        .header("user-agent", "openoxide")
        .send()
        .await
    {
        Ok(response) => response,
        Err(error) => {
            tracing::error!(%error, "GitHub manifest exchange request failed");
            return fail("could not reach GitHub");
        }
    };
    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        tracing::error!(%status, %body, "GitHub manifest exchange rejected");
        return fail("GitHub rejected the manifest code");
    }
    let conversion: Conversion = match response.json().await {
        Ok(conversion) => conversion,
        Err(error) => {
            tracing::error!(%error, "GitHub manifest exchange returned unreadable JSON");
            return fail("could not read GitHub's response");
        }
    };

    let service = match auto_di::resolve::<GitProviderService>().await {
        Ok(service) => service,
        Err(error) => {
            tracing::error!(%error, "git provider service unavailable");
            return fail("git provider service unavailable");
        }
    };
    let app_name = conversion
        .slug
        .clone()
        .or_else(|| conversion.name.clone())
        .unwrap_or_else(|| "github-app".to_string());
    let created = service
        .create(
            CreateProvider {
                name: conversion.name.unwrap_or_else(|| app_name.clone()),
                shared: false,
                credentials: ProviderCredentials::Github {
                    app_name: Some(app_name),
                    app_id: Some(conversion.id),
                    client_id: conversion.client_id,
                    client_secret: conversion.client_secret,
                    installation_id: None,
                    private_key: conversion.pem,
                },
            },
            organization_id,
        )
        .await;

    match created {
        Ok(_) => Response::builder()
            .status(302)
            .header("location", "/git-providers")
            .body(
                Full::<Bytes>::default()
                    .map_err(|error| match error {})
                    .boxed_unsync(),
            )
            .expect("static 302 response is always valid"),
        Err(error) => {
            tracing::error!(%error, "could not store the GitHub provider");
            fail("could not store the GitHub provider")
        }
    }
}
