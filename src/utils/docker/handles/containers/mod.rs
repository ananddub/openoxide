use crate::utils::docker::{DockerCli, DockerResult};

pub use create::ContainerCreate;
pub use exec::ExecBuilder;
pub use lifecycle::{
    ContainerKillBuilder, ContainerPauseBuilder, ContainerPortBuilder, ContainerRenameBuilder,
    ContainerRestartBuilder, ContainerRmBuilder, ContainerStartBuilder, ContainerStopBuilder,
    ContainerTopBuilder, ContainerUnpauseBuilder, ContainerUpdateBuilder, ContainerWaitBuilder,
};
pub use logs::LogsBuilder;
pub use prune::ContainerPrune;
pub use query::{ContainerQuery, RestartPolicy};
pub use stats::StatsBuilder;

pub struct ContainerHandle<'a>(pub(crate) &'a DockerCli);

impl<'a> ContainerHandle<'a> {
    pub fn ps(&self) -> ContainerQuery<'_> {
        ContainerQuery::new(self.0)
    }
    pub fn create(&self, image: impl Into<String>) -> ContainerCreate<'_> {
        ContainerCreate::new(self.0, image)
    }
    pub fn exec(&self, id: impl Into<String>) -> ExecBuilder<'_> {
        ExecBuilder::new(self.0, id)
    }
    pub fn logs(&self, id: impl Into<String>) -> LogsBuilder<'_> {
        LogsBuilder::new(self.0, id)
    }
    pub fn stats(&self, id: impl Into<String>) -> StatsBuilder<'_> {
        StatsBuilder::new(self.0, id)
    }
    pub fn prune(&self) -> ContainerPrune<'_> {
        ContainerPrune::new(self.0)
    }

    pub fn start(&self, id: impl Into<String>) -> ContainerStartBuilder<'_> {
        ContainerStartBuilder::new(self.0, id)
    }
    pub fn stop(&self, id: impl Into<String>) -> ContainerStopBuilder<'_> {
        ContainerStopBuilder::new(self.0, id)
    }
    pub fn restart(&self, id: impl Into<String>) -> ContainerRestartBuilder<'_> {
        ContainerRestartBuilder::new(self.0, id)
    }
    pub fn pause(&self, id: impl Into<String>) -> ContainerPauseBuilder<'_> {
        ContainerPauseBuilder::new(self.0, id)
    }
    pub fn unpause(&self, id: impl Into<String>) -> ContainerUnpauseBuilder<'_> {
        ContainerUnpauseBuilder::new(self.0, id)
    }
    pub fn kill(&self, id: impl Into<String>) -> ContainerKillBuilder<'_> {
        ContainerKillBuilder::new(self.0, id)
    }
    pub fn rm(&self, id: impl Into<String>) -> ContainerRmBuilder<'_> {
        ContainerRmBuilder::new(self.0, id)
    }
    pub fn rename(
        &self,
        id: impl Into<String>,
        name: impl Into<String>,
    ) -> ContainerRenameBuilder<'_> {
        ContainerRenameBuilder::new(self.0, id, name)
    }
    pub fn update(&self, id: impl Into<String>) -> ContainerUpdateBuilder<'_> {
        ContainerUpdateBuilder::new(self.0, id)
    }
    pub fn wait(&self, id: impl Into<String>) -> ContainerWaitBuilder<'_> {
        ContainerWaitBuilder::new(self.0, id)
    }
    pub fn port(&self, id: impl Into<String>) -> ContainerPortBuilder<'_> {
        ContainerPortBuilder::new(self.0, id)
    }
    pub fn top(&self, id: impl Into<String>) -> ContainerTopBuilder<'_> {
        ContainerTopBuilder::new(self.0, id)
    }
    pub async fn inspect(&self, id: impl AsRef<str>) -> DockerResult<crate::utils::docker::ContainerInspect> {
        let out = self.0.run(["container", "inspect", id.as_ref()]).await?;
        let mut json: Vec<crate::utils::docker::ContainerInspect> = serde_json::from_str(&out.stdout)?;
        Ok(json.pop().unwrap_or_default())
    }

    pub async fn inspect_raw(&self, id: impl AsRef<str>) -> DockerResult<serde_json::Value> {
        let out = self.0.run(["container", "inspect", id.as_ref()]).await?;
        let mut json: Vec<serde_json::Value> = serde_json::from_str(&out.stdout)?;
        Ok(json.pop().unwrap_or_default())
    }
}

pub mod create;
pub mod exec;
pub mod lifecycle;
pub mod logs;
pub mod prune;
pub mod query;
pub mod stats;
#[cfg(test)]
pub mod tests;
