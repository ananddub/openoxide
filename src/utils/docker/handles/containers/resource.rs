use crate::utils::docker::{
    DockerCli, DockerResult,
    handles::{
        ContainerInspectBuilder, ContainerKillBuilder, ContainerPauseBuilder,
        ContainerRemoveBuilder, ContainerRestartBuilder, ContainerStartBuilder,
        ContainerStopBuilder, ContainerUnpauseBuilder, ContainerUpdateBuilder, LogsBuilder,
    },
};

pub struct ContainerResource<'a> {
    cli: &'a DockerCli,
    id: String,
}

impl<'a> ContainerResource<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self {
        Self { cli, id: id.into() }
    }

    pub fn start(&self) -> ContainerStartBuilder<'a> {
        ContainerStartBuilder::new(self.cli, self.id.clone())
    }

    pub fn stop(&self) -> ContainerStopBuilder<'a> {
        ContainerStopBuilder::new(self.cli, self.id.clone())
    }

    pub fn restart(&self) -> ContainerRestartBuilder<'a> {
        ContainerRestartBuilder::new(self.cli, self.id.clone())
    }

    pub fn kill(&self) -> ContainerKillBuilder<'a> {
        ContainerKillBuilder::new(self.cli, self.id.clone())
    }

    pub fn pause(&self) -> ContainerPauseBuilder<'a> {
        ContainerPauseBuilder::new(self.cli, self.id.clone())
    }

    pub fn unpause(&self) -> ContainerUnpauseBuilder<'a> {
        ContainerUnpauseBuilder::new(self.cli, self.id.clone())
    }

    pub fn remove(&self) -> ContainerRemoveBuilder<'a> {
        ContainerRemoveBuilder::new(self.cli, self.id.clone())
    }

    pub fn update(&self) -> ContainerUpdateBuilder<'a> {
        ContainerUpdateBuilder::new(self.cli, self.id.clone())
    }

    pub fn logs(&self) -> LogsBuilder<'a> {
        LogsBuilder::new(self.cli, self.id.clone())
    }

    pub fn inspect_command(&self) -> ContainerInspectBuilder<'a> {
        ContainerInspectBuilder::new(self.cli, self.id.clone())
    }

    pub async fn inspect(&self) -> DockerResult<crate::utils::docker::ContainerInspect> {
        self.cli.containers().inspect(&self.id).await
    }

    pub async fn inspect_raw(&self) -> DockerResult<serde_json::Value> {
        self.cli.containers().inspect_raw(&self.id).await
    }
}
