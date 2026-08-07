use super::filter::VolumeFilter;
use crate::docker::{DockerCli, DockerResult, VolumeSummary};

// ── VolumeQuery ───────────────────────────────────────────────────────────────

/// Fluent builder for `docker volume ls`.
pub struct VolumeQuery<'a> {
    cli: &'a DockerCli,
    filters: Vec<VolumeFilter>,
}

impl<'a> VolumeQuery<'a> {
    pub(super) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            filters: vec![],
        }
    }

    pub fn filter(mut self, f: VolumeFilter) -> Self {
        self.filters.push(f);
        self
    }
    pub fn filters(mut self, fs: impl IntoIterator<Item = VolumeFilter>) -> Self {
        self.filters.extend(fs);
        self
    }

    fn args(&self) -> Vec<String> {
        let mut a = vec![
            "volume".into(),
            "ls".into(),
            "--format".into(),
            "{{json .}}".into(),
        ];
        for f in &self.filters {
            a.extend(["--filter".into(), f.to_string()]);
        }
        a
    }

    pub async fn list(self) -> DockerResult<Vec<VolumeSummary>> {
        let args = self.args();
        let refs: Vec<&str> = args.iter().map(String::as_str).collect();
        self.cli.json_lines(&refs).await
    }

    pub async fn exists(self) -> DockerResult<bool> {
        Ok(!self.list().await?.is_empty())
    }
}

// ── VolumeCreate ──────────────────────────────────────────────────────────────

/// Fluent builder for `docker volume create`.
///
/// Terminal method: [`.run()`] returns the volume name.
pub struct VolumeCreate<'a> {
    cli: &'a DockerCli,
    name: String,
    driver: Option<String>,
    labels: Vec<(String, String)>,
    opts: Vec<(String, String)>,
    extra: Vec<String>,
}

impl<'a> VolumeCreate<'a> {
    pub(super) fn new(cli: &'a DockerCli, name: impl Into<String>) -> Self {
        Self {
            cli,
            name: name.into(),
            driver: None,
            labels: vec![],
            opts: vec![],
            extra: vec![],
        }
    }

    pub fn driver(mut self, d: super::super::types::VolumeDriver) -> Self {
        self.driver = Some(d.as_str().to_string());
        self
    }
    pub fn label(mut self, k: impl Into<String>, v: impl Into<String>) -> Self {
        self.labels.push((k.into(), v.into()));
        self
    }
    /// Pass a driver-specific option (`--opt key=value`).
    ///
    /// Common uses for the built-in `local` driver:
    /// ```
    /// .opt("type", "nfs")
    /// .opt("o", "addr=192.168.1.10,rw")
    /// .opt("device", ":/export/data")
    /// ```
    pub fn opt(mut self, k: impl Into<String>, v: impl Into<String>) -> Self {
        self.opts.push((k.into(), v.into()));
        self
    }

    fn build_args(&self) -> Vec<String> {
        let mut a = vec!["volume".to_string(), "create".to_string()];
        if let Some(d) = &self.driver {
            a.extend(["--driver".into(), d.clone()]);
        }
        for (k, v) in &self.labels {
            a.extend(["--label".into(), format!("{k}={v}")]);
        }
        for (k, v) in &self.opts {
            a.extend(["--opt".into(), format!("{k}={v}")]);
        }
        a.extend(self.extra.clone());
        a.push(self.name.clone());
        a
    }

    /// Execute `docker volume create` and return the volume name.
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
    fn volume_create_nfs_args() {
        let cli = fake();
        let args = VolumeCreate::new(&cli, "nfs-data")
            .driver(crate::docker::VolumeDriver::Local)
            .opt("type", "nfs")
            .opt("o", "addr=10.0.0.1,rw")
            .opt("device", ":/exports/data")
            .label("managed-by", "rustploy")
            .build_args();

        assert!(args.contains(&"--driver".to_string()));
        assert!(args.contains(&"local".to_string()));
        assert!(args.contains(&"type=nfs".to_string()));
        assert!(args.contains(&"o=addr=10.0.0.1,rw".to_string()));
        assert!(args.contains(&"managed-by=rustploy".to_string()));
        assert_eq!(args.last(), Some(&"nfs-data".to_string()));
    }
}
