use std::env;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CollectionMode {
    #[default]
    Auto,
    Cgroup,
    Stream,
}

impl FromStr for CollectionMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "auto" => Ok(Self::Auto),
            "cgroup" => Ok(Self::Cgroup),
            "stream" => Ok(Self::Stream),
            invalid => Err(format!(
                "invalid COLLECTION_MODE: {invalid:?}, expected 'auto', 'cgroup', or 'stream'"
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub server_id: i64,
    pub database_url: String,
    pub grpc_port: u16,
    pub refresh_rate: u64,
    pub retention_days: i64,
    pub panel_url: String,
    pub metrics_token: String,
    pub docker_socket: String,
    pub include_containers: Vec<String>,
    pub exclude_containers: Vec<String>,
    pub collection_mode: CollectionMode,
    pub rollup_samples: u32,
}

fn parse_var<T: std::str::FromStr>(key: &str, default: T) -> Result<T, String> {
    match env::var(key) {
        Ok(raw) if !raw.trim().is_empty() => raw
            .trim()
            .parse::<T>()
            .map_err(|_| format!("{key} is set to {raw:?}, which is not a valid value")),
        _ => Ok(default),
    }
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let config = Self {
            server_id: parse_var("SERVER_ID", 1i64)?,
            database_url: env::var("MONITOR_DATABASE_URL")
                .unwrap_or_else(|_| "sqlite://monitor.db".to_string()),
            grpc_port: parse_var("GRPC_PORT", 50051u16)?,
            refresh_rate: parse_var("REFRESH_RATE", 60u64)?,
            retention_days: parse_var("RETENTION_DAYS", 7i64)?,
            panel_url: env::var("RUSTPLOY_SERVER_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:4000".to_string()),
            metrics_token: env::var("METRICS_TOKEN").unwrap_or_default(),
            docker_socket: env::var("DOCKER_SOCKET")
                .unwrap_or_else(|_| "/var/run/docker.sock".to_string()),
            include_containers: crate::filter::parse_patterns(
                &env::var("INCLUDE_CONTAINERS").unwrap_or_default(),
            ),
            exclude_containers: crate::filter::parse_patterns(
                &env::var("EXCLUDE_CONTAINERS").unwrap_or_default(),
            ),
            collection_mode: parse_var("COLLECTION_MODE", CollectionMode::Auto)?,
            rollup_samples: parse_var("ROLLUP_SAMPLES", 1u32)?,
        };

        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<(), String> {
        if self.server_id <= 0 {
            return Err(format!(
                "SERVER_ID must be a positive integer, got {}",
                self.server_id
            ));
        }

        if self.refresh_rate == 0 {
            return Err("REFRESH_RATE must be at least 1 second".to_string());
        }

        if self.retention_days <= 0 {
            return Err(format!(
                "RETENTION_DAYS must be a positive integer, got {}",
                self.retention_days
            ));
        }

        if !self.database_url.starts_with("sqlite:") {
            return Err(format!(
                "MONITOR_DATABASE_URL must be a sqlite:// URL, got {:?}",
                self.database_url
            ));
        }

        if !self.panel_url.starts_with("http://") && !self.panel_url.starts_with("https://") {
            return Err(format!(
                "RUSTPLOY_SERVER_URL must start with http:// or https://, got {:?}",
                self.panel_url
            ));
        }

        if self.metrics_token.trim().is_empty() {
            return Err(
                "METRICS_TOKEN must be set; anonymous metric pushes are disabled".to_string(),
            );
        }

        Ok(())
    }

    pub fn container_filter(&self) -> crate::filter::ContainerFilter {
        crate::filter::ContainerFilter::new(
            self.include_containers.clone(),
            self.exclude_containers.clone(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> Config {
        Config {
            server_id: 1,
            database_url: "sqlite://monitor.db".into(),
            grpc_port: 50051,
            refresh_rate: 60,
            retention_days: 7,
            panel_url: "http://127.0.0.1:4000".into(),
            metrics_token: "test-monitoring-token".into(),
            docker_socket: "/var/run/docker.sock".into(),
            include_containers: Vec::new(),
            exclude_containers: Vec::new(),
            collection_mode: CollectionMode::Auto,
            rollup_samples: 1,
        }
    }

    #[test]
    fn accepts_a_sane_config() {
        assert!(base().validate().is_ok());
    }

    #[test]
    fn rejects_zero_refresh_rate() {
        let mut cfg = base();
        cfg.refresh_rate = 0;
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn rejects_non_sqlite_database_url() {
        let mut cfg = base();
        cfg.database_url = "postgres://localhost/db".into();
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn rejects_panel_url_without_scheme() {
        let mut cfg = base();
        cfg.panel_url = "127.0.0.1:4000".into();
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn rejects_an_empty_metrics_token() {
        let mut cfg = base();
        cfg.metrics_token.clear();
        assert!(cfg.validate().is_err());
    }
}
