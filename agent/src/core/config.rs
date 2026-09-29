use auto_di::singleton;
use std::env;

fn parse_var<T: std::str::FromStr>(key: &str, default: T) -> Result<T, String> {
    match env::var(key) {
        Ok(raw) if !raw.trim().is_empty() => raw
            .trim()
            .parse::<T>()
            .map_err(|_| format!("{key} is set to {raw:?}, which is not a valid value")),
        _ => Ok(default),
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub server_id: i64,
    pub grpc_port: u16,
    pub metrics_token: String,
    pub docker_socket: String,
    pub max_upload_bytes: u64,
    pub upload_timeout_seconds: u64,
    pub upload_roots: Vec<String>,
    pub tls_cert: Option<String>,
    pub tls_key: Option<String>,
    /// CA that signs the panel's client cert; when set the agent requires and
    /// verifies a client certificate (mTLS).
    pub tls_client_ca: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let config = Self {
            server_id: parse_var("SERVER_ID", 1i64)?,
            grpc_port: parse_var("GRPC_PORT", 50051u16)?,
            metrics_token: env::var("METRICS_TOKEN").unwrap_or_default(),
            docker_socket: env::var("DOCKER_SOCKET")
                .unwrap_or_else(|_| "/var/run/docker.sock".to_string()),
            max_upload_bytes: parse_var("MAX_UPLOAD_BYTES", 1024u64 * 1024 * 1024)?,
            upload_timeout_seconds: parse_var("UPLOAD_TIMEOUT_SECONDS", 300u64)?,
            upload_roots: env::var("UPLOAD_ROOTS")
                .unwrap_or_else(|_| "/tmp,/etc/openoxide,/app/workspace".into())
                .split(',')
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(str::to_owned)
                .collect(),
            tls_cert: env::var("AGENT_TLS_CERT")
                .ok()
                .filter(|v| !v.trim().is_empty()),
            tls_key: env::var("AGENT_TLS_KEY")
                .ok()
                .filter(|v| !v.trim().is_empty()),
            tls_client_ca: env::var("AGENT_TLS_CLIENT_CA")
                .ok()
                .filter(|v| !v.trim().is_empty()),
        };

        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<(), String> {
        if self.server_id == 0 || self.server_id < -1 {
            return Err(format!(
                "SERVER_ID must be a positive integer or -1 for the local Agent, got {}",
                self.server_id
            ));
        }

        if self.metrics_token.trim().is_empty() {
            return Err(
                "METRICS_TOKEN must be set; anonymous metric pushes are disabled".to_string(),
            );
        }
        if self.max_upload_bytes == 0 || self.upload_timeout_seconds == 0 {
            return Err("upload size and timeout limits must be positive".into());
        }
        if self.upload_roots.is_empty()
            || self
                .upload_roots
                .iter()
                .any(|root| !std::path::Path::new(root).is_absolute())
        {
            return Err("UPLOAD_ROOTS must contain absolute paths".into());
        }
        if self.tls_cert.is_some() != self.tls_key.is_some() {
            return Err("AGENT_TLS_CERT and AGENT_TLS_KEY must be configured together".into());
        }

        Ok(())
    }
}

#[singleton]
impl Config {
    pub fn new() -> Result<Self, String> {
        Self::from_env()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> Config {
        Config {
            server_id: 1,
            grpc_port: 50051,
            metrics_token: "test-agent-token".into(),
            docker_socket: "/var/run/docker.sock".into(),
            max_upload_bytes: 1024 * 1024,
            upload_timeout_seconds: 300,
            upload_roots: vec!["/tmp".into()],
            tls_cert: None,
            tls_key: None,
        }
    }

    #[test]
    fn accepts_a_sane_config() {
        assert!(base().validate().is_ok());
    }

    #[test]
    fn rejects_an_empty_metrics_token() {
        let mut cfg = base();
        cfg.metrics_token.clear();
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn rejects_relative_upload_roots() {
        let mut cfg = base();
        cfg.upload_roots = vec!["relative/path".into()];
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn rejects_tls_cert_without_key() {
        let mut cfg = base();
        cfg.tls_cert = Some("/tmp/cert.pem".into());
        assert!(cfg.validate().is_err());
    }
}
