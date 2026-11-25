use bytes::Bytes;
use http_body_util::{BodyStream, Empty};
use hyper::{Request, StatusCode};
use hyper_util::rt::TokioIo;
use serde::de::DeserializeOwned;
use std::pin::Pin;
use std::sync::Arc;
use tokio::net::UnixStream;
use tokio::sync::Mutex;
use tokio_stream::{Stream, StreamExt};

use crate::core::config::Config;
use crate::error::DockerError;
use auto_di::singleton;

pub type ByteStream = Pin<Box<dyn Stream<Item = Result<Bytes, DockerError>> + Send>>;

type Sender = hyper::client::conn::http1::SendRequest<Empty<Bytes>>;
type Connection = hyper::client::conn::http1::Connection<TokioIo<UnixStream>, Empty<Bytes>>;

/// Highest Engine API version this agent targets. Docker 29 dropped support for
/// anything below v1.44 (it answers HTTP 400 "client version 1.41 is too old"),
/// while older daemons reject versions above their own maximum — so the real
/// version is negotiated per daemon and capped here.
const MAX_API_VERSION: (u32, u32) = (1, 44);
/// Used only when the daemon's version cannot be parsed.
const FALLBACK_API_VERSION: &str = "v1.44";
const MAX_RESPONSE_BYTES: usize = 32 * 1024 * 1024;

/// One persistent connection to the Docker Engine, serialized behind a mutex:
/// the panel keeps a single gRPC connection to the agent and the agent keeps
/// a single connection to docker.sock. Long-lived streams (/stats, /events)
/// take a dedicated connection instead so they never block queries.
#[derive(Clone)]
pub struct DockerApi {
    inner: Arc<Inner>,
}

struct Inner {
    socket_path: String,
    shared: Mutex<Option<Sender>>,
    /// Negotiated once; a failure (daemon not up yet) is retried next call.
    api_version: tokio::sync::OnceCell<String>,
}

/// Borrows the shared sender; returns it to the slot on drop unless the
/// caller marks it dead first.
struct Guard {
    inner: Arc<Inner>,
    sender: Option<Sender>,
}

impl Drop for Guard {
    fn drop(&mut self) {
        let Some(sender) = self.sender.take() else {
            return;
        };
        let inner = self.inner.clone();
        // Drop may be synchronous: hand the sender back on a trivial task.
        tokio::spawn(async move {
            *inner.shared.lock().await = Some(sender);
        });
    }
}

impl Guard {
    fn sender(&mut self) -> &mut Sender {
        self.sender.as_mut().expect("guard holds a sender")
    }

    /// Connection is unusable: drop the sender, which tears down its driver
    /// task and socket, and keep it out of the slot.
    fn discard(&mut self) {
        drop(self.sender.take());
    }
}

impl DockerApi {
    pub fn from_socket(socket_path: impl Into<String>) -> Self {
        Self {
            inner: Arc::new(Inner {
                socket_path: socket_path.into(),
                shared: Mutex::new(None),
                api_version: tokio::sync::OnceCell::new(),
            }),
        }
    }

    pub async fn get_raw(&self, path: &str) -> Result<Bytes, DockerError> {
        self.request(hyper::Method::GET, path).await
    }

    /// POST with an empty body, collecting the (bounded) response — the
    /// shape of Docker's RPC-style endpoints like /images/create.
    pub async fn post_raw(&self, path: &str) -> Result<Bytes, DockerError> {
        self.request(hyper::Method::POST, path).await
    }

    async fn request(&self, method: hyper::Method, path: &str) -> Result<Bytes, DockerError> {
        let uri = self.versioned(path).await?;
        let mut guard = self.checkout().await?;
        let response = match self.send(guard.sender(), method, &uri).await {
            Ok(response) => response,
            Err(error) => {
                guard.discard();
                return Err(error);
            }
        };
        let keep_alive = !is_close(&response);
        let status = response.status();
        let body = match collect_limited(response.into_body()).await {
            Ok(body) => body,
            Err(error) => {
                guard.discard();
                return Err(error);
            }
        };
        if keep_alive {
            drop(guard); // returned to the slot
        } else {
            guard.discard();
        }
        if !status.is_success() {
            let detail = String::from_utf8_lossy(&body).trim().to_string();
            return Err(match status {
                StatusCode::NOT_FOUND => DockerError::NotFound { uri, detail },
                _ => DockerError::ApiStatus {
                    status,
                    uri,
                    detail,
                },
            });
        }
        Ok(body)
    }

    pub async fn get_json<T: DeserializeOwned>(&self, path: &str) -> Result<T, DockerError> {
        let body = self.get_raw(path).await?;
        serde_json::from_slice(&body).map_err(|source| DockerError::JsonDecode {
            path: path.to_string(),
            source,
        })
    }

    /// Long-lived responses (stats streams, /events) own a dedicated
    /// connection for their whole lifetime.
    pub async fn get_stream(&self, path: &str) -> Result<ByteStream, DockerError> {
        self.open_stream(hyper::Method::GET, path).await
    }

    /// Dedicated-connection POST for long-running streamed endpoints such as
    /// /images/create, whose progress body can outlive the shared connection's
    /// request serialization.
    pub async fn post_stream(&self, path: &str) -> Result<ByteStream, DockerError> {
        self.open_stream(hyper::Method::POST, path).await
    }

    async fn open_stream(
        &self,
        method: hyper::Method,
        path: &str,
    ) -> Result<ByteStream, DockerError> {
        let uri = self.versioned(path).await?;
        let (mut sender, connection) = self.open().await?;
        let driver = tokio::spawn(async move {
            if let Err(error) = connection.await {
                tracing::debug!(%error, "docker API stream connection closed");
            }
        });

        match self.send(&mut sender, method, &uri).await {
            Ok(response) => {
                let status = response.status();
                if !status.is_success() {
                    let body = collect_limited(response.into_body()).await?;
                    let detail = String::from_utf8_lossy(&body).trim().to_string();
                    return Err(DockerError::ApiStatus {
                        status,
                        uri,
                        detail,
                    });
                }
                let chunks = BodyStream::new(response.into_body()).map(|frame| match frame {
                    Ok(frame) => Ok(frame.data_ref().cloned().unwrap_or_default()),
                    Err(e) => Err(DockerError::Stream(e.to_string())),
                });
                // The driver must outlive this stream: dropping the handle
                // detaches it, and it ends on its own once the response body
                // is dropped.
                drop(driver);
                Ok(Box::pin(chunks))
            }
            Err(error) => {
                driver.abort();
                Err(error)
            }
        }
    }

    pub async fn ping(&self) -> Result<(), DockerError> {
        self.get_raw("/_ping").await.map(|_| ())
    }

    async fn checkout(&self) -> Result<Guard, DockerError> {
        {
            let mut shared = self.inner.shared.lock().await;
            if let Some(sender) = shared.as_mut()
                && !sender.is_closed()
            {
                let sender = shared.take().expect("slot holds a sender");
                return Ok(Guard {
                    inner: self.inner.clone(),
                    sender: Some(sender),
                });
            }
            *shared = None;
        }
        let sender = self.connect().await?;
        Ok(Guard {
            inner: self.inner.clone(),
            sender: Some(sender),
        })
    }

    /// Open the single shared connection and park its driver task.
    async fn connect(&self) -> Result<Sender, DockerError> {
        let (sender, connection) = self.open().await?;
        tokio::spawn(async move {
            if let Err(error) = connection.await {
                tracing::debug!(%error, "docker API connection closed");
            }
        });
        Ok(sender)
    }

    async fn open(&self) -> Result<(Sender, Connection), DockerError> {
        let stream = UnixStream::connect(&self.inner.socket_path)
            .await
            .map_err(|source| DockerError::SocketConnect {
                path: self.inner.socket_path.clone(),
                source,
            })?;
        hyper::client::conn::http1::handshake(TokioIo::new(stream))
            .await
            .map_err(|e| DockerError::Handshake(e.to_string()))
    }

    async fn send(
        &self,
        sender: &mut Sender,
        method: hyper::Method,
        uri: &str,
    ) -> Result<hyper::Response<hyper::body::Incoming>, DockerError> {
        let uri = uri.to_string();
        let request = Request::builder()
            .method(method)
            .uri(&uri)
            .header("Host", "docker")
            .body(Empty::<Bytes>::new())
            .map_err(|e| DockerError::RequestFailed {
                uri: uri.clone(),
                message: e.to_string(),
            })?;
        sender
            .ready()
            .await
            .map_err(|e| DockerError::RequestFailed {
                uri: uri.clone(),
                message: e.to_string(),
            })?;
        if sender.is_closed() {
            return Err(DockerError::RequestFailed {
                uri,
                message: "connection closed".to_string(),
            });
        }
        sender
            .send_request(request)
            .await
            .map_err(|e| DockerError::RequestFailed {
                uri: uri.clone(),
                message: e.to_string(),
            })
    }
}

#[singleton]
impl DockerApi {
    pub fn new(config: Arc<Config>) -> Self {
        Self::from_socket(&config.docker_socket)
    }
}

impl DockerApi {
    /// `/v1.xx/<path>` for the negotiated API version.
    async fn versioned(&self, path: &str) -> Result<String, DockerError> {
        let version = self
            .inner
            .api_version
            .get_or_try_init(|| self.negotiate_version())
            .await?;
        Ok(format!("/{version}{path}"))
    }

    /// Ask the daemon for its API version over an unversioned `/version` (the
    /// one endpoint every Engine answers regardless of version) on a throwaway
    /// connection, so a failure never poisons the shared sender.
    async fn negotiate_version(&self) -> Result<String, DockerError> {
        let (mut sender, connection) = self.open().await?;
        let driver = tokio::spawn(async move {
            let _ = connection.await;
        });
        let result = async {
            let response = self
                .send(&mut sender, hyper::Method::GET, "/version")
                .await?;
            let status = response.status();
            let body = collect_limited(response.into_body()).await?;
            if !status.is_success() {
                return Err(DockerError::ApiStatus {
                    status,
                    uri: "/version".to_string(),
                    detail: String::from_utf8_lossy(&body).trim().to_string(),
                });
            }
            Ok(serde_json::from_slice::<serde_json::Value>(&body)
                .ok()
                .and_then(|value| value.get("ApiVersion")?.as_str().map(str::to_owned)))
        }
        .await;
        driver.abort();
        let version = match result? {
            Some(server) => clamp_api_version(&server),
            None => FALLBACK_API_VERSION.to_string(),
        };
        tracing::info!(api_version = %version, "negotiated Docker Engine API version");
        Ok(version)
    }
}

fn clamp_api_version(server: &str) -> String {
    let parsed = server
        .trim()
        .trim_start_matches('v')
        .split_once('.')
        .and_then(|(major, minor)| Some((major.parse::<u32>().ok()?, minor.parse::<u32>().ok()?)));
    match parsed {
        Some(server) => {
            let chosen = server.min(MAX_API_VERSION);
            format!("v{}.{}", chosen.0, chosen.1)
        }
        None => FALLBACK_API_VERSION.to_string(),
    }
}

fn is_close(response: &hyper::Response<hyper::body::Incoming>) -> bool {
    response
        .headers()
        .get(hyper::header::CONNECTION)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.eq_ignore_ascii_case("close"))
}

async fn collect_limited(body: hyper::body::Incoming) -> Result<Bytes, DockerError> {
    use http_body_util::BodyStream;
    use tokio_stream::StreamExt;

    let mut stream = BodyStream::new(body);
    let mut buffer = Vec::new();

    while let Some(frame) = stream.next().await {
        let frame = frame.map_err(|e| DockerError::Stream(e.to_string()))?;

        if let Some(chunk) = frame.data_ref() {
            if buffer.len() + chunk.len() > MAX_RESPONSE_BYTES {
                return Err(DockerError::PayloadTooLarge {
                    max_bytes: MAX_RESPONSE_BYTES,
                });
            }
            buffer.extend_from_slice(chunk);
        }
    }

    Ok(Bytes::from(buffer))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_version_is_capped_to_what_the_daemon_supports() {
        assert_eq!(clamp_api_version("1.52"), "v1.44"); // Docker 29
        assert_eq!(clamp_api_version("1.44"), "v1.44");
        assert_eq!(clamp_api_version("v1.41"), "v1.41"); // old daemon: never above it
        assert_eq!(clamp_api_version("1.24"), "v1.24");
        assert_eq!(clamp_api_version("garbage"), "v1.44");
    }

    #[tokio::test]
    async fn missing_socket_is_a_clear_error() {
        let api = DockerApi::from_socket("/nonexistent/docker.sock");
        let error = api.ping().await.expect_err("should fail");
        assert!(
            matches!(error, DockerError::SocketConnect { .. }),
            "unexpected error: {error:?}"
        );
    }
}
