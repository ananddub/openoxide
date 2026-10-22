use std::time::Duration;

use auto_di::singleton;
use serde::{Deserialize, Serialize};
use tokio::{net::TcpStream, time::timeout};

#[derive(Debug, Deserialize)]
pub struct DomainDiagnosticDto {
    pub host: String,
    pub port: Option<u16>,
    pub https: bool,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DomainDiagnosticResponseDto {
    pub host: String,
    pub resolved_addresses: Vec<String>,
    pub dns_ok: bool,
    pub tcp_ok: bool,
    pub http_ok: bool,
    pub http_status: Option<u16>,
    pub error: Option<String>,
}

pub struct NetworkDiagnosticService {
    client: reqwest::Client,
}

#[singleton]
impl NetworkDiagnosticService {
    fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .connect_timeout(Duration::from_secs(5))
                .timeout(Duration::from_secs(10))
                .redirect(reqwest::redirect::Policy::limited(5))
                .build()
                .expect("network diagnostic client must build"),
        }
    }

    pub async fn diagnose(&self, input: DomainDiagnosticDto) -> DomainDiagnosticResponseDto {
        let host = input.host.trim().trim_end_matches('.').to_ascii_lowercase();
        let port = input.port.unwrap_or(if input.https { 443 } else { 80 });
        let mut errors = Vec::new();
        let resolved_addresses = match tokio::net::lookup_host((host.as_str(), port)).await {
            Ok(addresses) => addresses.map(|address| address.ip().to_string()).collect(),
            Err(error) => {
                errors.push(format!("DNS: {error}"));
                Vec::new()
            }
        };
        let dns_ok = !resolved_addresses.is_empty();
        let tcp_ok = match timeout(
            Duration::from_secs(5),
            TcpStream::connect((host.as_str(), port)),
        )
        .await
        {
            Ok(Ok(_)) => true,
            Ok(Err(error)) => {
                errors.push(format!("TCP: {error}"));
                false
            }
            Err(_) => {
                errors.push("TCP: connection timed out".into());
                false
            }
        };
        let scheme = if input.https { "https" } else { "http" };
        let path = input.path.unwrap_or_else(|| "/".into());
        let url = format!("{scheme}://{host}:{port}/{}", path.trim_start_matches('/'));
        let (http_ok, http_status) = match self.client.get(url).send().await {
            Ok(response) => (true, Some(response.status().as_u16())),
            Err(error) => {
                errors.push(format!("HTTP: {error}"));
                (false, None)
            }
        };
        DomainDiagnosticResponseDto {
            host,
            resolved_addresses,
            dns_ok,
            tcp_ok,
            http_ok,
            http_status,
            error: (!errors.is_empty()).then(|| errors.join("; ")),
        }
    }
}
