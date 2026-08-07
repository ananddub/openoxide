use crate::utils::docker::{DockerCli, DockerResult, handles::VolumeRmBuilder};

pub type VolumeRemoveBuilder<'a> = VolumeRmBuilder<'a>;

pub struct VolumeResource<'a> {
    cli: &'a DockerCli,
    name: String,
}

impl<'a> VolumeResource<'a> {
    pub(crate) fn new(cli: &'a DockerCli, name: impl Into<String>) -> Self {
        Self {
            cli,
            name: name.into(),
        }
    }

    pub fn remove(&self) -> VolumeRemoveBuilder<'a> {
        VolumeRemoveBuilder::new(self.cli, self.name.clone())
    }

    pub async fn inspect(&self) -> DockerResult<crate::utils::docker::VolumeInspect> {
        self.cli.volumes().inspect(&self.name).await
    }

    pub async fn inspect_raw(&self) -> DockerResult<serde_json::Value> {
        self.cli.volumes().inspect_raw(&self.name).await
    }
}
