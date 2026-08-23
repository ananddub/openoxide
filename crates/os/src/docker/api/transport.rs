use super::super::DockerError;
use super::API_VERSION;
use bytes::Bytes;
use http_body_util::BodyExt;
use hyper::{Request, StatusCode};
use hyper_util::rt::TokioIo;
use serde::de::DeserializeOwned;
use tokio::net::UnixStream;
use tokio::sync::mpsc;

#[derive(Clone, Debug)]
pub struct DockerApi {
    socket_path: String,
}

impl DockerApi {
    pub fn new(socket_path: impl Into<String>) -> Self {
        Self {
            socket_path: socket_path.into(),
        }
    }

    pub async fn request(
        &self,
        method: &str,
        path: &str,
        body: Option<Bytes>,
    ) -> Result<Bytes, DockerError> {
        self.request_with_type(method, path, body, "application/json")
            .await
    }

    pub async fn request_with_auth(
        &self,
        method: &str,
        path: &str,
        body: Option<Bytes>,
        auth: Option<&str>,
    ) -> Result<Bytes, DockerError> {
        self.request_with_type_and_auth(method, path, body, "application/json", auth)
            .await
    }

    pub async fn request_with_type(
        &self,
        method: &str,
        path: &str,
        body: Option<Bytes>,
        content_type: &str,
    ) -> Result<Bytes, DockerError> {
        self.request_with_type_and_auth(method, path, body, content_type, None)
            .await
    }

    async fn request_with_type_and_auth(
        &self,
        method: &str,
        path: &str,
        body: Option<Bytes>,
        content_type: &str,
        auth: Option<&str>,
    ) -> Result<Bytes, DockerError> {
        let stream = UnixStream::connect(&self.socket_path)
            .await
            .map_err(|e| DockerError::Ssh(format!("docker socket {}: {e}", self.socket_path)))?;
        let (mut sender, connection) = hyper::client::conn::http1::handshake(TokioIo::new(stream))
            .await
            .map_err(|e| DockerError::Ssh(e.to_string()))?;
        tokio::spawn(async move {
            let _ = connection.await;
        });
        let uri = format!("/{API_VERSION}{path}");
        let mut request = Request::builder()
            .method(method)
            .uri(&uri)
            .header("Host", "docker")
            .header("Content-Type", content_type);
        if let Some(auth) = auth {
            request = request.header("X-Registry-Auth", auth);
        }
        let request = request
            .body(http_body_util::Full::new(body.unwrap_or_default()).boxed())
            .map_err(|e| DockerError::Ssh(e.to_string()))?;
        let response = sender
            .send_request(request)
            .await
            .map_err(|e| DockerError::Ssh(e.to_string()))?;
        let status = response.status();
        let payload = response
            .into_body()
            .collect()
            .await
            .map_err(|e| DockerError::Ssh(e.to_string()))?
            .to_bytes();
        if !status.is_success() && status != StatusCode::NO_CONTENT {
            return Err(DockerError::Ssh(format!(
                "Docker API {method} {uri} returned {status}: {}",
                String::from_utf8_lossy(&payload).trim()
            )));
        }
        Ok(payload)
    }

    pub async fn get_json<T: DeserializeOwned>(&self, path: &str) -> Result<T, DockerError> {
        serde_json::from_slice(&self.request("GET", path, None).await?).map_err(Into::into)
    }

    pub async fn request_stream(
        &self,
        method: &str,
        path: &str,
        body: Option<Bytes>,
        sender: mpsc::Sender<crate::docker::DockerStreamEvent>,
    ) -> Result<u16, DockerError> {
        self.request_stream_with_type(method, path, body, "application/json", sender)
            .await
    }

    pub async fn request_stream_with_type(
        &self,
        method: &str,
        path: &str,
        body: Option<Bytes>,
        content_type: &str,
        sender: mpsc::Sender<crate::docker::DockerStreamEvent>,
    ) -> Result<u16, DockerError> {
        let stream = UnixStream::connect(&self.socket_path)
            .await
            .map_err(|e| DockerError::Ssh(format!("docker socket {}: {e}", self.socket_path)))?;
        let (mut client, connection) = hyper::client::conn::http1::handshake(TokioIo::new(stream))
            .await
            .map_err(|e| DockerError::Ssh(e.to_string()))?;
        tokio::spawn(async move {
            let _ = connection.await;
        });
        let uri = format!("/{API_VERSION}{path}");
        let request = Request::builder()
            .method(method)
            .uri(&uri)
            .header("Host", "docker")
            .header("Content-Type", content_type)
            .body(http_body_util::Full::new(body.unwrap_or_default()).boxed())
            .map_err(|e| DockerError::Ssh(e.to_string()))?;
        let response = client
            .send_request(request)
            .await
            .map_err(|e| DockerError::Ssh(e.to_string()))?;
        let status = response.status();
        if !status.is_success() {
            return Err(DockerError::Ssh(format!(
                "Docker API {method} {uri} returned {status}"
            )));
        }
        let mut body = response.into_body();
        let mut buffered = Vec::new();
        while let Some(frame) = body.frame().await {
            let frame = frame.map_err(|e| DockerError::Ssh(e.to_string()))?;
            if let Ok(data) = frame.into_data() {
                buffered.extend_from_slice(&data);
                while let Some((kind, payload, consumed)) = multiplex_frame(&buffered) {
                    let event = if kind == 2 {
                        crate::docker::DockerStreamEvent::Stderr(payload.to_vec())
                    } else {
                        crate::docker::DockerStreamEvent::Stdout(payload.to_vec())
                    };
                    if sender.send(event).await.is_err() {
                        return Ok(status.as_u16());
                    }
                    buffered.drain(..consumed);
                }
                if buffered.len() >= 8 && !looks_multiplexed(&buffered) {
                    let raw = std::mem::take(&mut buffered);
                    if sender
                        .send(crate::docker::DockerStreamEvent::Stdout(raw))
                        .await
                        .is_err()
                    {
                        return Ok(status.as_u16());
                    }
                }
            }
        }
        if !buffered.is_empty() {
            let _ = sender
                .send(crate::docker::DockerStreamEvent::Stdout(buffered))
                .await;
        }
        Ok(status.as_u16())
    }
}

fn looks_multiplexed(data: &[u8]) -> bool {
    data.len() >= 8 && matches!(data[0], 0..=2) && data[1..4] == [0, 0, 0]
}

fn multiplex_frame(data: &[u8]) -> Option<(u8, &[u8], usize)> {
    if !looks_multiplexed(data) {
        return None;
    }
    let size = u32::from_be_bytes(data[4..8].try_into().ok()?) as usize;
    let end = 8usize.checked_add(size)?;
    (data.len() >= end).then(|| (data[0], &data[8..end], end))
}
