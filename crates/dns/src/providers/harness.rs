//! Shared plumbing for the provider tests: recorded replies from each provider's real API,
//! mounted on a local mock server, plus the requests the client actually sent.
//!
//! Nothing here reaches production — `harness` is only compiled for `cfg(test)`. The tests
//! run over plain HTTP against a port the mock server picked, so no TLS crypto provider is
//! involved and no live credentials are needed.

use std::sync::Once;

use serde_json::Value;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

static CRYPTO: Once = Once::new();

/// Recorded replies for one provider, keyed by scenario name. Each document is the body the
/// real API answered, so a test both pins the request the client builds and the row it reads
/// back from that exact shape.
pub(crate) fn scenario(provider: &str, name: &str) -> Value {
    let file = format!(
        "{}/src/providers/tests/{provider}.json",
        env!("CARGO_MANIFEST_DIR")
    );
    let text = std::fs::read_to_string(&file)
        .unwrap_or_else(|error| panic!("cannot read {file}: {error}"));
    let document: Value = serde_json::from_str(&text)
        .unwrap_or_else(|error| panic!("{file} is not valid JSON: {error}"));
    document
        .get(name)
        .cloned()
        .unwrap_or_else(|| panic!("{file} has no scenario {name:?}"))
}

pub(crate) fn client() -> reqwest::Client {
    CRYPTO.call_once(|| {
        // Only ever reached from a test binary; the panel installs ring in `main.rs` and this
        // crate never installs anything itself.
        let _ = rustls::crypto::ring::default_provider().install_default();
    });
    reqwest::Client::new()
}

/// Answers one verb + route (query strings ignored) with a recorded JSON body.
pub(crate) async fn ok(server: &MockServer, verb: &str, route: &str, body: Value) {
    respond(server, verb, route, 200, body).await;
}

/// Same, for the case where the provider answers a failure status.
pub(crate) async fn respond(
    server: &MockServer,
    verb: &str,
    route: &str,
    status: u16,
    body: Value,
) {
    Mock::given(method(verb))
        .and(path(route))
        .respond_with(ResponseTemplate::new(status).set_body_json(body))
        .mount(server)
        .await;
}

/// A plain-text answer, for the endpoints that do not return JSON (OVH's clock probe).
pub(crate) async fn ok_text(server: &MockServer, verb: &str, route: &str, text: &str) {
    Mock::given(method(verb))
        .and(path(route))
        .respond_with(ResponseTemplate::new(200).set_body_string(text.to_string()))
        .mount(server)
        .await;
}

/// How many times the client hit one verb + route.
pub(crate) async fn hits(server: &MockServer, verb: &str, route: &str) -> usize {
    sent(server, verb, route).await.len()
}

/// The JSON bodies the client sent to one verb + route, in order. Reading the request rather
/// than matching it in a `Mock::given` means an unexpected payload shows up as a diff of the
/// two documents instead of an unexplained 404.
pub(crate) async fn sent(server: &MockServer, verb: &str, route: &str) -> Vec<Value> {
    requests(server, verb, route)
        .await
        .into_iter()
        .map(|request| {
            serde_json::from_slice(&request.body)
                .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&request.body).into()))
        })
        .collect()
}

pub(crate) async fn header(
    server: &MockServer,
    verb: &str,
    route: &str,
    name: &str,
) -> Option<String> {
    requests(server, verb, route)
        .await
        .first()
        .and_then(|request| request.headers.get(name))
        .and_then(|value| value.to_str().ok())
        .map(str::to_string)
}

async fn requests(server: &MockServer, verb: &str, route: &str) -> Vec<Request> {
    server
        .received_requests()
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|request| request.method.as_str() == verb && request.url.path() == route)
        .collect()
}
