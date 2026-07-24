use super::filter::ServiceFilter;
use crate::utils::docker::{DockerCli, DockerOutput, DockerResult, ServiceSummary};

// ── ServiceQuery ──────────────────────────────────────────────────────────────

/// Fluent builder for `docker service ls`.
pub struct ServiceQuery<'a> {
    cli: &'a DockerCli,
    filters: Vec<ServiceFilter>,
}

impl<'a> ServiceQuery<'a> {
    pub(super) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            filters: vec![],
        }
    }

    pub fn filter(mut self, f: ServiceFilter) -> Self {
        self.filters.push(f);
        self
    }
    pub fn filters(mut self, fs: impl IntoIterator<Item = ServiceFilter>) -> Self {
        self.filters.extend(fs);
        self
    }

    fn args(&self) -> Vec<String> {
        let mut a = vec![
            "service".into(),
            "ls".into(),
            "--format".into(),
            "{{json .}}".into(),
        ];
        for f in &self.filters {
            a.extend(["--filter".into(), f.to_string()]);
        }
        a
    }

    pub async fn list(self) -> DockerResult<Vec<ServiceSummary>> {
        let args = self.args();
        let refs: Vec<&str> = args.iter().map(String::as_str).collect();
        self.cli.json_lines(&refs).await
    }

    pub async fn exists(self) -> DockerResult<bool> {
        Ok(!self.list().await?.is_empty())
    }
}

// ── ServiceUpdate ─────────────────────────────────────────────────────────────

/// Fluent builder for `docker service update`.
///
/// Terminal method: [`.run()`].
pub struct ServiceUpdate<'a> {
    cli: &'a DockerCli,
    name: String,
    image: Option<String>,
    replicas: Option<u32>,
    label_add: Vec<(String, String)>,
    label_rm: Vec<String>,
    env_add: Vec<(String, String)>,
    env_rm: Vec<String>,
    constraint_add: Vec<String>,
    constraint_rm: Vec<String>,
    limit_memory: Option<String>,
    limit_cpu: Option<String>,
    reserve_memory: Option<String>,
    reserve_cpu: Option<String>,
    update_parallelism: Option<u32>,
    update_delay: Option<String>,
    rollback: bool,
    force: bool,
    extra: Vec<String>,
}

impl<'a> ServiceUpdate<'a> {
    pub(super) fn new(cli: &'a DockerCli, name: impl Into<String>) -> Self {
        Self {
            cli,
            name: name.into(),
            image: None,
            replicas: None,
            label_add: vec![],
            label_rm: vec![],
            env_add: vec![],
            env_rm: vec![],
            constraint_add: vec![],
            constraint_rm: vec![],
            limit_memory: None,
            limit_cpu: None,
            reserve_memory: None,
            reserve_cpu: None,
            update_parallelism: None,
            update_delay: None,
            rollback: false,
            force: false,
            extra: vec![],
        }
    }

    pub fn image(mut self, v: impl Into<String>) -> Self {
        self.image = Some(v.into());
        self
    }
    pub fn replicas(mut self, n: u32) -> Self {
        self.replicas = Some(n);
        self
    }

    pub fn label_add(mut self, k: impl Into<String>, v: impl Into<String>) -> Self {
        self.label_add.push((k.into(), v.into()));
        self
    }
    pub fn label_rm(mut self, k: impl Into<String>) -> Self {
        self.label_rm.push(k.into());
        self
    }

    pub fn env_add(mut self, k: impl Into<String>, v: impl Into<String>) -> Self {
        self.env_add.push((k.into(), v.into()));
        self
    }
    pub fn env_rm(mut self, k: impl Into<String>) -> Self {
        self.env_rm.push(k.into());
        self
    }

    pub fn constraint_add(mut self, c: impl Into<String>) -> Self {
        self.constraint_add.push(c.into());
        self
    }
    pub fn constraint_rm(mut self, c: impl Into<String>) -> Self {
        self.constraint_rm.push(c.into());
        self
    }

    pub fn limit_memory(mut self, v: impl Into<String>) -> Self {
        self.limit_memory = Some(v.into());
        self
    }
    pub fn limit_cpu(mut self, v: impl Into<String>) -> Self {
        self.limit_cpu = Some(v.into());
        self
    }
    pub fn reserve_memory(mut self, v: impl Into<String>) -> Self {
        self.reserve_memory = Some(v.into());
        self
    }
    pub fn reserve_cpu(mut self, v: impl Into<String>) -> Self {
        self.reserve_cpu = Some(v.into());
        self
    }

    pub fn update_parallelism(mut self, n: u32) -> Self {
        self.update_parallelism = Some(n);
        self
    }
    pub fn update_delay(mut self, v: impl Into<String>) -> Self {
        self.update_delay = Some(v.into());
        self
    }

    /// Apply `--rollback` to revert the service to the previous spec.
    pub fn rollback(mut self) -> Self {
        self.rollback = true;
        self
    }
    /// Force an update even if nothing changed.
    pub fn force(mut self) -> Self {
        self.force = true;
        self
    }
    /// Pass any raw flag not yet covered.
    pub fn arg(mut self, v: impl Into<String>) -> Self {
        self.extra.push(v.into());
        self
    }

    fn build_args(&self) -> Vec<String> {
        let mut a = vec!["service".to_string(), "update".to_string()];

        if let Some(v) = &self.image {
            a.extend(["--image".into(), v.clone()]);
        }
        if let Some(n) = self.replicas {
            a.extend(["--replicas".into(), n.to_string()]);
        }

        for (k, v) in &self.label_add {
            a.extend(["--label-add".into(), format!("{k}={v}")]);
        }
        for k in &self.label_rm {
            a.extend(["--label-rm".into(), k.clone()]);
        }
        for (k, v) in &self.env_add {
            a.extend(["--env-add".into(), format!("{k}={v}")]);
        }
        for k in &self.env_rm {
            a.extend(["--env-rm".into(), k.clone()]);
        }
        for c in &self.constraint_add {
            a.extend(["--constraint-add".into(), c.clone()]);
        }
        for c in &self.constraint_rm {
            a.extend(["--constraint-rm".into(), c.clone()]);
        }

        if let Some(v) = &self.limit_memory {
            a.extend(["--limit-memory".into(), v.clone()]);
        }
        if let Some(v) = &self.limit_cpu {
            a.extend(["--limit-cpu".into(), v.clone()]);
        }
        if let Some(v) = &self.reserve_memory {
            a.extend(["--reserve-memory".into(), v.clone()]);
        }
        if let Some(v) = &self.reserve_cpu {
            a.extend(["--reserve-cpu".into(), v.clone()]);
        }

        if let Some(n) = self.update_parallelism {
            a.extend(["--update-parallelism".into(), n.to_string()]);
        }
        if let Some(v) = &self.update_delay {
            a.extend(["--update-delay".into(), v.clone()]);
        }

        if self.rollback {
            a.push("--rollback".into());
        }
        if self.force {
            a.push("--force".into());
        }

        a.extend(self.extra.clone());
        a.push(self.name.clone());
        a
    }
    pub fn print(self) -> String {
        let args = self.build_args();
        args.join(" ")
    }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let args = self.build_args();
        let refs: Vec<&str> = args.iter().map(String::as_str).collect();
        self.cli.run(refs).await
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::docker::DockerCli;

    fn fake() -> DockerCli {
        DockerCli::new_local()
    }

    #[test]
    fn update_builds_correct_args() {
        let cli = fake();
        let args = ServiceUpdate::new(&cli, "myapp")
            .image("myapp:v2")
            .replicas(3)
            .label_add("version", "v2")
            .label_rm("old-label")
            .env_add("FEATURE", "on")
            .limit_memory("512m")
            .update_parallelism(1)
            .update_delay("10s")
            .force()
            .build_args();

        assert_eq!(&args[0..2], &["service", "update"]);
        assert!(args.contains(&"--image".to_string()));
        assert!(args.contains(&"myapp:v2".to_string()));
        assert!(args.contains(&"--replicas".to_string()));
        assert!(args.contains(&"3".to_string()));
        assert!(args.contains(&"version=v2".to_string()));
        assert!(args.contains(&"--label-rm".to_string()));
        assert!(args.contains(&"old-label".to_string()));
        assert!(args.contains(&"FEATURE=on".to_string()));
        assert!(args.contains(&"--limit-memory".to_string()));
        assert!(args.contains(&"--force".to_string()));
        assert_eq!(args.last(), Some(&"myapp".to_string()));
    }

    #[test]
    fn service_test() {
        // let docker = DockerCli::new_local();
        // let data  = docker.services().list().filter(ServiceFilter::name("myservice")).print();
        // println!("Service data: {}", data);
    }
}
