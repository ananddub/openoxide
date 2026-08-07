use crate::docker::{
    ContainerSummary, DockerCli, DockerResult, core::ArgBuilder, query::filter::ContainerFilter,
};
use std::fmt;

// ── ContainerQuery ────────────────────────────────────────────────────────────

pub struct ContainerQuery<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) args: ArgBuilder,
}

impl<'a> ContainerQuery<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["container", "ls", "--format", "{{json .}}"]),
        }
    }
    pub fn all(mut self) -> Self {
        self.args.flag("--all");
        self
    }
    pub fn filter(mut self, f: ContainerFilter) -> Self {
        self.args.filter(f);
        self
    }
    pub fn filters(mut self, fs: impl IntoIterator<Item = ContainerFilter>) -> Self {
        for f in fs {
            self.args.filter(f);
        }
        self
    }

    /// Dry-run: print the docker command without executing.
    pub fn print(&self) -> String {
        self.args.preview()
    }

    pub async fn list(self) -> DockerResult<Vec<ContainerSummary>> {
        self.cli.execute_json_lines(&self.args).await
    }
    pub async fn count(self) -> DockerResult<usize> {
        Ok(self.list().await?.len())
    }
    pub async fn exists(self) -> DockerResult<bool> {
        Ok(!self.list().await?.is_empty())
    }
}
crate::impl_builder_opts!(ContainerQuery);

// ── RestartPolicy ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestartPolicy {
    No,
    Always,
    OnFailure(u32),
    UnlessStopped,
}

impl fmt::Display for RestartPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::No => write!(f, "no"),
            Self::Always => write!(f, "always"),
            Self::OnFailure(n) => write!(f, "on-failure:{n}"),
            Self::UnlessStopped => write!(f, "unless-stopped"),
        }
    }
}
