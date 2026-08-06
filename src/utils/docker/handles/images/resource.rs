use crate::utils::docker::{DockerCli, DockerResult, handles::ImageRmBuilder};

pub type ImageRemoveBuilder<'a> = ImageRmBuilder<'a>;

pub struct ImageResource<'a> {
    cli: &'a DockerCli,
    id: String,
}

impl<'a> ImageResource<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self {
        Self { cli, id: id.into() }
    }

    pub fn remove(&self) -> ImageRemoveBuilder<'a> {
        ImageRemoveBuilder::new(self.cli, self.id.clone())
    }

    pub async fn inspect(&self) -> DockerResult<crate::utils::docker::ImageInspect> {
        self.cli.images().inspect(&self.id).await
    }

    pub async fn inspect_raw(&self) -> DockerResult<serde_json::Value> {
        self.cli.images().inspect_raw(&self.id).await
    }
}
