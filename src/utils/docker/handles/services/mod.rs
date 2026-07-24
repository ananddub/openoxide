use crate::utils::docker::{DockerResult, client::DockerCli};

pub use create::ServiceCreateBuilder;
pub use list::ServiceListBuilder;
pub use logs::ServiceLogsBuilder;
pub use ps::ServicePsBuilder;
pub use remove::ServiceRemoveBuilder;
pub use update::{ServiceRollbackBuilder, ServiceScaleBuilder, ServiceUpdateBuilder};

// ── ServicesHandle ──────────────────────────────────────────────────────────

pub struct ServicesHandle<'a> {
    cli: &'a DockerCli,
}

impl<'a> ServicesHandle<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self { cli }
    }

    pub fn list(&self) -> ServiceListBuilder<'a> {
        ServiceListBuilder::new(self.cli)
    }

    pub fn update(&self, name: impl Into<String>) -> ServiceUpdateBuilder<'a> {
        ServiceUpdateBuilder::new(self.cli, name)
    }

    pub fn create(&self, image: impl Into<String>) -> ServiceCreateBuilder<'a> {
        ServiceCreateBuilder::new(self.cli, image)
    }

    pub fn remove(&self, name: impl Into<String>) -> ServiceRemoveBuilder<'a> {
        ServiceRemoveBuilder::new(self.cli, name)
    }

    pub fn ps(&self, name: impl Into<String>) -> ServicePsBuilder<'a> {
        ServicePsBuilder::new(self.cli, name)
    }

    pub fn logs(&self, name: impl Into<String>) -> ServiceLogsBuilder<'a> {
        ServiceLogsBuilder::new(self.cli, name)
    }

    pub fn scale(&self) -> ServiceScaleBuilder<'a> {
        ServiceScaleBuilder::new(self.cli)
    }

    pub fn rollback(&self, name: impl Into<String>) -> ServiceRollbackBuilder<'a> {
        ServiceRollbackBuilder::new(self.cli, name)
    }

    pub async fn inspect(&self, name: impl AsRef<str>) -> DockerResult<serde_json::Value> {
        let out = self.cli.run(["service", "inspect", name.as_ref()]).await?;
        let mut json: Vec<serde_json::Value> = serde_json::from_str(&out.stdout)?;
        Ok(json.pop().unwrap_or_default())
    }
}

pub mod create;
pub mod list;
pub mod logs;
pub mod ps;
pub mod remove;
pub mod update;
