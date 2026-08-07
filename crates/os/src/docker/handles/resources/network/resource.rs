use crate::docker::{DockerCli, DockerResult, handles::NetworkRmBuilder};

pub type NetworkRemoveBuilder<'a> = NetworkRmBuilder<'a>;

pub struct NetworkResource<'a> {
    cli: &'a DockerCli,
    name: String,
}

impl<'a> NetworkResource<'a> {
    pub(crate) fn new(cli: &'a DockerCli, name: impl Into<String>) -> Self {
        Self {
            cli,
            name: name.into(),
        }
    }

    pub fn remove(&self) -> NetworkRemoveBuilder<'a> {
        NetworkRemoveBuilder::new(self.cli, self.name.clone())
    }

    pub async fn inspect(&self) -> DockerResult<crate::docker::NetworkInspect> {
        self.cli.networks().inspect(&self.name).await
    }

    pub async fn inspect_raw(&self) -> DockerResult<serde_json::Value> {
        self.cli.networks().inspect_raw(&self.name).await
    }
}
