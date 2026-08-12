use super::filter::NetworkFilter;
use crate::docker::{DockerCli, DockerResult, NetworkSummary};

// ── NetworkQuery ──────────────────────────────────────────────────────────────

/// Fluent builder for `docker network ls`.
pub struct NetworkQuery<'a> {
    cli: &'a DockerCli,
    filters: Vec<NetworkFilter>,
}

impl<'a> NetworkQuery<'a> {
    pub(super) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            filters: vec![],
        }
    }

    pub fn filter(mut self, f: NetworkFilter) -> Self {
        self.filters.push(f);
        self
    }
    pub fn filters(mut self, fs: impl IntoIterator<Item = NetworkFilter>) -> Self {
        self.filters.extend(fs);
        self
    }

    fn args(&self) -> Vec<String> {
        let mut a = vec![
            "network".into(),
            "ls".into(),
            "--format".into(),
            "{{json .}}".into(),
        ];
        for f in &self.filters {
            a.extend(["--filter".into(), f.to_string()]);
        }
        a
    }

    pub async fn list(self) -> DockerResult<Vec<NetworkSummary>> {
        let args = self.args();
        let refs: Vec<&str> = args.iter().map(String::as_str).collect();
        self.cli.json_lines(&refs).await
    }

    pub async fn exists(self) -> DockerResult<bool> {
        Ok(!self.list().await?.is_empty())
    }
}

// ── NetworkCreate ─────────────────────────────────────────────────────────────

/// Fluent builder for `docker network create`.
///
/// Terminal method: [`.run()`] returns the network ID.
pub struct NetworkCreate<'a> {
    cli: &'a DockerCli,
    name: String,
    driver: Option<String>,
    subnet: Option<String>,
    gateway: Option<String>,
    labels: Vec<(String, String)>,
    opts: Vec<(String, String)>,
    attachable: bool,
    internal: bool,
    ipv6: bool,
    scope: Option<String>,
    extra: Vec<String>,
}

impl<'a> NetworkCreate<'a> {
    pub(super) fn new(cli: &'a DockerCli, name: impl Into<String>) -> Self {
        Self {
            cli,
            name: name.into(),
            driver: None,
            subnet: None,
            gateway: None,
            labels: vec![],
            opts: vec![],
            attachable: false,
            internal: false,
            ipv6: false,
            scope: None,
            extra: vec![],
        }
    }

    /// Set the network driver (default: `bridge`; use `overlay` for Swarm).
    pub fn driver(mut self, d: super::super::types::NetworkDriver) -> Self {
        self.driver = Some(d.as_str().to_string());
        self
    }
    pub fn subnet(mut self, v: impl Into<String>) -> Self {
        self.subnet = Some(v.into());
        self
    }
    pub fn gateway(mut self, v: impl Into<String>) -> Self {
        self.gateway = Some(v.into());
        self
    }

    pub fn label(mut self, k: impl Into<String>, v: impl Into<String>) -> Self {
        self.labels.push((k.into(), v.into()));
        self
    }
    /// Pass a driver-specific option (`--opt key=value`).
    pub fn opt(mut self, k: impl Into<String>, v: impl Into<String>) -> Self {
        self.opts.push((k.into(), v.into()));
        self
    }

    /// Allow containers outside the Swarm to attach (`--attachable`).
    pub fn attachable(mut self) -> Self {
        self.attachable = true;
        self
    }
    /// Restrict external access (`--internal`).
    pub fn internal(mut self) -> Self {
        self.internal = true;
        self
    }
    pub fn ipv6(mut self) -> Self {
        self.ipv6 = true;
        self
    }
    pub fn scope(mut self, s: crate::docker::NetworkScope) -> Self {
        self.scope = Some(s.to_string());
        self
    }

    fn build_args(&self) -> Vec<String> {
        let mut a = vec!["network".to_string(), "create".to_string()];
        if let Some(d) = &self.driver {
            a.extend(["--driver".into(), d.clone()]);
        }
        if let Some(s) = &self.subnet {
            a.extend(["--subnet".into(), s.clone()]);
        }
        if let Some(g) = &self.gateway {
            a.extend(["--gateway".into(), g.clone()]);
        }
        for (k, v) in &self.labels {
            a.extend(["--label".into(), format!("{k}={v}")]);
        }
        for (k, v) in &self.opts {
            a.extend(["--opt".into(), format!("{k}={v}")]);
        }
        if let Some(s) = &self.scope {
            a.extend(["--scope".into(), s.clone()]);
        }
        if self.attachable {
            a.push("--attachable".into());
        }
        if self.internal {
            a.push("--internal".into());
        }
        if self.ipv6 {
            a.push("--ipv6".into());
        }
        a.extend(self.extra.clone());
        a.push(self.name.clone());
        a
    }

    /// Execute `docker network create` and return the new network ID.
    pub async fn run(self) -> DockerResult<String> {
        let cli = self.cli;
        let args = self.build_args();
        let refs: Vec<&str> = args.iter().map(String::as_str).collect();
        Ok(cli.run(refs).await?.stdout.trim().to_string())
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::docker::DockerCli;

    fn fake() -> DockerCli {
        DockerCli::new_local()
    }

    #[test]
    fn network_create_builds_overlay_swarm_args() {
        let cli = fake();
        let args = NetworkCreate::new(&cli, "openoxide-network")
            .driver(crate::docker::NetworkDriver::Overlay)
            .attachable()
            .scope(crate::docker::NetworkScope::Swarm)
            .label("managed-by", "openoxide")
            .opt("encrypted", "")
            .build_args();

        assert!(args.contains(&"--driver".to_string()));
        assert!(args.contains(&"overlay".to_string()));
        assert!(args.contains(&"--attachable".to_string()));
        assert!(args.contains(&"--scope".to_string()));
        assert!(args.contains(&"swarm".to_string()));
        assert!(args.contains(&"managed-by=openoxide".to_string()));
        assert_eq!(args.last(), Some(&"openoxide-network".to_string()));
    }
}
