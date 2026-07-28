use crate::utils::docker::{DockerResult, client::DockerCli};

pub use create::ConfigCreateBuilder;
pub use list::ConfigListBuilder;
pub use remove::ConfigRemoveBuilder;

// ── ConfigsHandle ───────────────────────────────────────────────────────────

pub struct ConfigsHandle<'a> {
    cli: &'a DockerCli,
}

impl<'a> ConfigsHandle<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self { cli }
    }

    pub fn create(&self, name: impl Into<String>) -> ConfigCreateBuilder<'a> {
        ConfigCreateBuilder::new(self.cli, name)
    }

    pub fn remove(&self, name: impl Into<String>) -> ConfigRemoveBuilder<'a> {
        ConfigRemoveBuilder::new(self.cli, name)
    }

    pub fn list(&self) -> ConfigListBuilder<'a> {
        ConfigListBuilder::new(self.cli)
    }

    pub async fn inspect(
        &self,
        name: impl AsRef<str>,
    ) -> DockerResult<crate::utils::docker::ConfigInspect> {
        let out = self.cli.run(["config", "inspect", name.as_ref()]).await?;
        let mut json: Vec<crate::utils::docker::ConfigInspect> = serde_json::from_str(&out.stdout)?;
        Ok(json.pop().unwrap_or_default())
    }

    pub async fn inspect_raw(&self, name: impl AsRef<str>) -> DockerResult<serde_json::Value> {
        let out = self.cli.run(["config", "inspect", name.as_ref()]).await?;
        let mut json: Vec<serde_json::Value> = serde_json::from_str(&out.stdout)?;
        Ok(json.pop().unwrap_or_default())
    }
}

pub mod create;
pub mod list;
pub mod remove;
