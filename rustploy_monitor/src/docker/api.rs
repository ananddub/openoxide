use bytes::Bytes;
use http_body_util::{BodyStream, Empty};
use hyper::{Request, StatusCode};
use hyper_util::rt::TokioIo;
use serde::de::DeserializeOwned;
use std::pin::Pin;
use tokio::net::UnixStream;
use tokio_stream::{Stream, StreamExt};

/// A streaming response body, yielding raw chunks as the daemon sends them.
pub type ByteStream = Pin<Box<dyn Stream<Item = Result<Bytes, String>> + Send>>;

/// Docker API version to pin requests to. 1.41 ships with Docker 20.10 (2020),
/// old enough to work everywhere we care about and new enough for everything
/// the agent asks for.
const API_VERSION: &str = "v1.41";

/// Bytes we are willing to buffer from a single response. Log tails and stats
/// payloads are small; anything larger means something is wrong and we would
/// rather error than grow the heap without bound.
const MAX_RESPONSE_BYTES: usize = 32 * 1024 * 1024;

/// Minimal client for the Docker Engine API over its unix socket.
///
/// This replaces shelling out to the `docker` CLI, which cost 30 MB in the
/// image for two commands and a process spawn every collection cycle.
#[derive(Clone)]
pub struct DockerApi {
    socket_path: String,
}

impl DockerApi {
    pub fn new(socket_path: impl Into<String>) -> Self {
        Self {
            socket_path: socket_path.into(),
        }
    }

    /// Performs a GET and returns the raw response body.
    ///
    /// A fresh connection per request: these calls happen once a cycle, so
    /// pooling would add moving parts for no measurable gain.
    pub async fn get_raw(&self, path: &str) -> Result<Bytes, String> {
        let stream = UnixStream::connect(&self.socket_path)
            .await
            .map_err(|e| format!("could not connect to docker socket {}: {e}", self.socket_path))?;

        let (mut sender, connection) = hyper::client::conn::http1::handshake(TokioIo::new(stream))
            .await
            .map_err(|e| format!("docker API handshake failed: {e}"))?;

        // The connection task drives IO; it ends when the response completes.
        tokio::spawn(async move {
            if let Err(error) = connection.await {
                tracing::debug!(%error, "docker API connection closed");
            }
        });

        let uri = format!("/{API_VERSION}{path}");
        let request = Request::builder()
            .method("GET")
            .uri(&uri)
            // Required by HTTP/1.1; the daemon ignores the value on a unix socket.
            .header("Host", "docker")
            .body(Empty::<Bytes>::new())
            .map_err(|e| format!("could not build docker API request: {e}"))?;

        let response = sender
            .send_request(request)
            .await
            .map_err(|e| format!("docker API request to {uri} failed: {e}"))?;

        let status = response.status();
        let body = collect_limited(response.into_body()).await?;

        if !status.is_success() {
            let detail = String::from_utf8_lossy(&body);
            let detail = detail.trim();
            return Err(match status {
                StatusCode::NOT_FOUND => format!("docker API {uri}: not found ({detail})"),
                _ => format!("docker API {uri} returned {status}: {detail}"),
            });
        }

        Ok(body)
    }

    /// Performs a GET and decodes the response as JSON.
    pub async fn get_json<T: DeserializeOwned>(&self, path: &str) -> Result<T, String> {
        let body = self.get_raw(path).await?;
        serde_json::from_slice(&body)
            .map_err(|e| format!("could not decode docker API response from {path}: {e}"))
    }

    /// Opens a streaming GET and yields body chunks as the daemon writes them.
    ///
    /// Used for endpoints that never complete on their own — `/containers/{id}/stats`
    /// pushes a frame per second for as long as the connection is held. The caller
    /// owns reconnection: the stream simply ends when the daemon closes it.
    pub async fn get_stream(&self, path: &str) -> Result<ByteStream, String> {
        let stream = UnixStream::connect(&self.socket_path)
            .await
            .map_err(|e| format!("could not connect to docker socket {}: {e}", self.socket_path))?;

        let (mut sender, connection) = hyper::client::conn::http1::handshake(TokioIo::new(stream))
            .await
            .map_err(|e| format!("docker API handshake failed: {e}"))?;

        tokio::spawn(async move {
            if let Err(error) = connection.await {
                tracing::debug!(%error, "docker API stream connection closed");
            }
        });

        let uri = format!("/{API_VERSION}{path}");
        let request = Request::builder()
            .method("GET")
            .uri(&uri)
            .header("Host", "docker")
            .body(Empty::<Bytes>::new())
            .map_err(|e| format!("could not build docker API request: {e}"))?;

        let response = sender
            .send_request(request)
            .await
            .map_err(|e| format!("docker API request to {uri} failed: {e}"))?;

        let status = response.status();
        if !status.is_success() {
            let body = collect_limited(response.into_body()).await?;
            let detail = String::from_utf8_lossy(&body);
            return Err(format!(
                "docker API {uri} returned {status}: {}",
                detail.trim()
            ));
        }

        // Trailers carry no payload, so frames without data become empty chunks
        // the caller skips rather than an error.
        let chunks = BodyStream::new(response.into_body()).map(|frame| match frame {
            Ok(frame) => Ok(frame.data_ref().cloned().unwrap_or_default()),
            Err(e) => Err(format!("docker API stream read failed: {e}")),
        });

        Ok(Box::pin(chunks))
    }

    /// True when the daemon socket is reachable. Used at startup so a missing
    /// socket mount is reported once rather than as an error every cycle.
    pub async fn ping(&self) -> Result<(), String> {
        self.get_raw("/_ping").await.map(|_| ())
    }
}

/// Reads a body, refusing to buffer more than `MAX_RESPONSE_BYTES`.
async fn collect_limited(body: hyper::body::Incoming) -> Result<Bytes, String> {
    use http_body_util::BodyStream;
    use tokio_stream::StreamExt;

    let mut stream = BodyStream::new(body);
    let mut buffer = Vec::new();

    while let Some(frame) = stream.next().await {
        let frame = frame.map_err(|e| format!("docker API body read failed: {e}"))?;

        if let Some(chunk) = frame.data_ref() {
            if buffer.len() + chunk.len() > MAX_RESPONSE_BYTES {
                return Err(format!(
                    "docker API response exceeded {MAX_RESPONSE_BYTES} bytes"
                ));
            }
            buffer.extend_from_slice(chunk);
        }
    }

    Ok(Bytes::from(buffer))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn missing_socket_is_a_clear_error() {
        let api = DockerApi::new("/nonexistent/docker.sock");
        let error = api.ping().await.expect_err("should fail");
        assert!(
            error.contains("could not connect to docker socket"),
            "unexpected error: {error}"
        );
    }
}
