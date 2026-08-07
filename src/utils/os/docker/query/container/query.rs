use crate::utils::docker::query::filter::ContainerFilter;
use crate::utils::docker::{ContainerSummary, DockerCli, DockerResult};

/// Fluent builder for `docker container ls`.
pub struct ContainerQuery<'a> {
    cli: &'a DockerCli,
    all: bool,
    filters: Vec<ContainerFilter>,
}

impl<'a> ContainerQuery<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            all: false,
            filters: vec![],
        }
    }

    /// Include stopped containers (`--all`).
    pub fn all(mut self) -> Self {
        self.all = true;
        self
    }

    /// Add a single filter.
    pub fn filter(mut self, f: ContainerFilter) -> Self {
        self.filters.push(f);
        self
    }

    /// Add multiple filters at once.
    pub fn filters(mut self, fs: impl IntoIterator<Item = ContainerFilter>) -> Self {
        self.filters.extend(fs);
        self
    }

    fn args(&self) -> Vec<String> {
        let mut a = vec![
            "container".into(),
            "ls".into(),
            "--format".into(),
            "{{json .}}".into(),
        ];
        if self.all {
            a.push("--all".into());
        }
        for f in &self.filters {
            a.extend(["--filter".into(), f.to_string()]);
        }
        a
    }

    /// Execute and return matching containers.
    pub async fn list(self) -> DockerResult<Vec<ContainerSummary>> {
        let args = self.args();
        let refs: Vec<&str> = args.iter().map(String::as_str).collect();
        self.cli.json_lines(&refs).await
    }

    /// Return the count of matching containers.
    pub async fn count(self) -> DockerResult<usize> {
        Ok(self.list().await?.len())
    }

    /// Return `true` if at least one container matches.
    pub async fn exists(self) -> DockerResult<bool> {
        Ok(!self.list().await?.is_empty())
    }
}
