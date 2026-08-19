use std::collections::HashMap;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
        }
    }
}

pub struct SocatRequestBuilder {
    method: HttpMethod,
    path: String,
    query_params: Vec<(String, String)>,
    headers: HashMap<String, String>,
    body: Vec<u8>,
    socket_path: String,
}

impl SocatRequestBuilder {
    pub fn new(method: HttpMethod, path: impl Into<String>) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Host".to_string(), "localhost".to_string());
        Self {
            method,
            path: path.into(),
            query_params: Vec::new(),
            headers,
            body: Vec::new(),
            socket_path: "/var/run/docker.sock".to_string(),
        }
    }

    pub fn get(path: impl Into<String>) -> Self {
        Self::new(HttpMethod::Get, path)
    }

    pub fn post(path: impl Into<String>) -> Self {
        Self::new(HttpMethod::Post, path)
    }

    pub fn put(path: impl Into<String>) -> Self {
        Self::new(HttpMethod::Put, path)
    }

    pub fn delete(path: impl Into<String>) -> Self {
        Self::new(HttpMethod::Delete, path)
    }

    pub fn query(mut self, k: impl Into<String>, v: impl Into<String>) -> Self {
        self.query_params.push((k.into(), v.into()));
        self
    }

    pub fn header(mut self, k: impl Into<String>, v: impl Into<String>) -> Self {
        self.headers.insert(k.into(), v.into());
        self
    }

    pub fn socket_path(mut self, path: impl Into<String>) -> Self {
        self.socket_path = path.into();
        self
    }

    pub fn json<T: serde::Serialize>(mut self, value: &T) -> Self {
        if let Ok(json_bytes) = serde_json::to_vec(value) {
            self.headers
                .insert("Content-Type".to_string(), "application/json".to_string());
            self.headers
                .insert("Content-Length".to_string(), json_bytes.len().to_string());
            self.body = json_bytes;
        }
        self
    }

    pub fn body(mut self, body: impl Into<Vec<u8>>) -> Self {
        let b = body.into();
        self.headers
            .insert("Content-Length".to_string(), b.len().to_string());
        self.body = b;
        self
    }

    pub fn build_full_path(&self) -> String {
        if self.query_params.is_empty() {
            self.path.clone()
        } else {
            let query_str = self
                .query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&");
            format!("{}?{}", self.path, query_str)
        }
    }

    pub fn build_http_bytes(&self) -> Vec<u8> {
        let full_path = self.build_full_path();
        let mut req_str = format!("{} {} HTTP/1.1\r\n", self.method.as_str(), full_path);
        for (k, v) in &self.headers {
            req_str.push_str(&format!("{}: {}\r\n", k, v));
        }
        req_str.push_str("\r\n");

        let mut bytes = req_str.into_bytes();
        bytes.extend_from_slice(&self.body);
        bytes
    }

    pub async fn send_on(&self, stream: &mut UnixStream) -> std::io::Result<String> {
        let req_bytes = self.build_http_bytes();
        stream.write_all(&req_bytes).await?;

        let mut buf = vec![0u8; 4096];
        let n = stream.read(&mut buf).await?;
        Ok(String::from_utf8_lossy(&buf[..n]).to_string())
    }

    pub async fn send(self) -> std::io::Result<String> {
        let mut stream = UnixStream::connect(&self.socket_path).await?;
        self.send_on(&mut stream).await
    }

    pub async fn upgrade_on(mut self, stream: &mut UnixStream) -> std::io::Result<String> {
        self.headers
            .insert("Connection".to_string(), "Upgrade".to_string());
        self.headers
            .insert("Upgrade".to_string(), "tcp".to_string());

        let req_bytes = self.build_http_bytes();
        stream.write_all(&req_bytes).await?;

        let mut header_buf = [0u8; 1024];
        let n = stream.read(&mut header_buf).await?;
        let header_str = String::from_utf8_lossy(&header_buf[..n]).to_string();
        if !header_str.contains("101") {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed HTTP socket upgrade: {}", header_str),
            ));
        }
        Ok(header_str)
    }

    pub async fn upgrade(self) -> std::io::Result<(UnixStream, String)> {
        let mut stream = UnixStream::connect(&self.socket_path).await?;
        let header_str = self.upgrade_on(&mut stream).await?;
        Ok((stream, header_str))
    }
}
