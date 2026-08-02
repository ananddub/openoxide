use std::env;

/// Every tunable the agent reads, resolved once at startup.
///
/// Previously these were scattered across `main.rs` as inline `env::var` calls
/// with silent `unwrap_or` fallbacks, so a typo in a var name looked identical
/// to "not configured". Loading them in one place lets us validate up front and
/// fail loudly instead of running half-configured.
#[derive(Debug, Clone)]
pub struct Config {
    /// Identifies this agent's host in metrics sent to the panel.
    pub server_id: i64,
    /// SQLite URL for the agent's local metric store.
    pub database_url: String,
    /// Port the gRPC query server listens on.
    pub grpc_port: u16,
    /// Seconds between collection cycles.
    pub refresh_rate: u64,
    /// Days of metrics to keep before the cleanup task deletes them.
    pub retention_days: i64,
    /// Base URL of the rustploy panel, used to push container metrics.
    pub panel_url: String,
    /// Shared secret sent with pushes so the panel can identify this agent.
    pub metrics_token: String,
    /// Unix socket of the docker daemon the agent talks to.
    pub docker_socket: String,
    /// Comma-separated container name patterns to monitor. Empty means all
    /// not excluded. `*` is the wildcard.
    pub include_containers: Vec<String>,
    /// Comma-separated container name patterns to never monitor.
    pub exclude_containers: Vec<String>,
    /// How container metrics are collected: `auto`, `cgroup` or `stream`.
    pub collection_mode: String,
    /// Samples per rollup window. High-density hosts set this to shrink what
    /// reaches storage: 300 samples at a 60 s cadence is a 5-minute window,
    /// collapsed to an average + peak pair.
    pub rollup_samples: u32,
}

/// Reads an env var as `T`, falling back to `default` when unset. A value that
/// is set but unparseable is an error rather than a silent fallback — that case
/// is always a misconfiguration worth surfacing.
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
    /// Loads configuration from the environment and validates it.
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
            collection_mode: env::var("COLLECTION_MODE")
                .unwrap_or_else(|_| "auto".to_string())
                .to_ascii_lowercase(),
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

        if !matches!(self.collection_mode.as_str(), "auto" | "cgroup" | "stream") {
            return Err(format!(
                "COLLECTION_MODE must be auto, cgroup or stream, got {:?}",
                self.collection_mode
            ));
        }

        Ok(())
    }

    /// Panel endpoint that receives batched container metric pushes.
    pub fn container_metrics_endpoint(&self) -> String {
        format!(
            "{}/api/monitoring/containers/batch",
            self.panel_url.trim_end_matches('/')
        )
    }

    /// Filter describing which containers this agent collects.
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
            metrics_token: String::new(),
            docker_socket: "/var/run/docker.sock".into(),
            include_containers: Vec::new(),
            exclude_containers: Vec::new(),
            collection_mode: "auto".into(),
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
    fn builds_container_endpoint_without_double_slash() {
        let mut cfg = base();
        cfg.panel_url = "http://panel:4000/".into();
        assert_eq!(
            cfg.container_metrics_endpoint(),
            "http://panel:4000/api/monitoring/containers/batch"
        );
    }
}
