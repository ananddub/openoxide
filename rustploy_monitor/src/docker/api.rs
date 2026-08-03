use bytes::Bytes;
use http_body_util::{BodyStream, Empty};
use hyper::{Request, StatusCode};
use hyper_util::rt::TokioIo;
use serde::de::DeserializeOwned;
use std::pin::Pin;
use tokio::net::UnixStream;
use tokio_stream::{Stream, StreamExt};

use crate::error::DockerError;

pub type ByteStream = Pin<Box<dyn Stream<Item = Result<Bytes, DockerError>> + Send>>;

const API_VERSION: &str = "v1.41";
const MAX_RESPONSE_BYTES: usize = 32 * 1024 * 1024;

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

    pub async fn get_raw(&self, path: &str) -> Result<Bytes, DockerError> {
        let stream = UnixStream::connect(&self.socket_path)
            .await
            .map_err(|source| DockerError::SocketConnect {
                path: self.socket_path.clone(),
                source,
            })?;

        let (mut sender, connection) = hyper::client::conn::http1::handshake(TokioIo::new(stream))
            .await
            .map_err(|e| DockerError::Handshake(e.to_string()))?;

        tokio::spawn(async move {
            if let Err(error) = connection.await {
                tracing::debug!(%error, "docker API connection closed");
            }
        });

        let uri = format!("/{API_VERSION}{path}");
        let request = Request::builder()
            .method("GET")
            .uri(&uri)
            .header("Host", "docker")
            .body(Empty::<Bytes>::new())
            .map_err(|e| DockerError::RequestFailed {
                uri: uri.clone(),
                message: e.to_string(),
            })?;

        let response = sender
            .send_request(request)
            .await
            .map_err(|e| DockerError::RequestFailed {
                uri: uri.clone(),
                message: e.to_string(),
            })?;

        let status = response.status();
        let body = collect_limited(response.into_body()).await?;

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

    pub async fn get_stream(&self, path: &str) -> Result<ByteStream, DockerError> {
        let stream = UnixStream::connect(&self.socket_path)
            .await
            .map_err(|source| DockerError::SocketConnect {
                path: self.socket_path.clone(),
                source,
            })?;

        let (mut sender, connection) = hyper::client::conn::http1::handshake(TokioIo::new(stream))
            .await
            .map_err(|e| DockerError::Handshake(e.to_string()))?;

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
            .map_err(|e| DockerError::RequestFailed {
                uri: uri.clone(),
                message: e.to_string(),
            })?;

        let response = sender
            .send_request(request)
            .await
            .map_err(|e| DockerError::RequestFailed {
                uri: uri.clone(),
                message: e.to_string(),
            })?;

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

        Ok(Box::pin(chunks))
    }

    pub async fn ping(&self) -> Result<(), DockerError> {
        self.get_raw("/_ping").await.map(|_| ())
    }
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

    #[tokio::test]
    async fn missing_socket_is_a_clear_error() {
        let api = DockerApi::new("/nonexistent/docker.sock");
        let error = api.ping().await.expect_err("should fail");
        assert!(
            matches!(error, DockerError::SocketConnect { .. }),
            "unexpected error: {error:?}"
        );
    }
}
