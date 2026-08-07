use crate::docker::DockerCli;
use std::path::Path;

pub use build::BuildBuilder;
pub use lifecycle::{
    ImageHistoryBuilder, ImageImportBuilder, ImageLoadBuilder, ImagePushBuilder, ImageRmBuilder,
    ImageSaveBuilder, ImageTagBuilder,
};
pub use prune::ImagePrune;
pub use pull::PullBuilder;
pub use query::ImageQuery;
pub use resource::{ImageRemoveBuilder, ImageResource};

pub struct ImageHandle<'a>(pub(crate) &'a DockerCli);

impl<'a> ImageHandle<'a> {
    pub fn list(&self) -> ImageQuery<'_> {
        ImageQuery::new(self.0)
    }
    pub fn build(&self, context: impl AsRef<Path>) -> BuildBuilder<'_> {
        BuildBuilder::new(self.0, context)
    }
    pub fn pull(&self, image: impl Into<String>) -> PullBuilder<'_> {
        PullBuilder::new(self.0, image)
    }
    pub fn prune(&self) -> ImagePrune<'_> {
        ImagePrune::new(self.0)
    }

    pub fn push(&self, image: impl Into<String>) -> ImagePushBuilder<'_> {
        ImagePushBuilder::new(self.0, image)
    }
    pub fn rm(&self, image: impl Into<String>) -> ImageRmBuilder<'_> {
        ImageRmBuilder::new(self.0, image)
    }
    pub fn tag(&self, source: impl Into<String>, target: impl Into<String>) -> ImageTagBuilder<'_> {
        ImageTagBuilder::new(self.0, source, target)
    }
    pub fn history(&self, image: impl Into<String>) -> ImageHistoryBuilder<'_> {
        ImageHistoryBuilder::new(self.0, image)
    }
    pub fn save(&self, image: impl Into<String>) -> ImageSaveBuilder<'_> {
        ImageSaveBuilder::new(self.0, image)
    }
    pub fn load(&self) -> ImageLoadBuilder<'_> {
        ImageLoadBuilder::new(self.0)
    }
    pub fn import(&self, source: impl Into<String>) -> ImageImportBuilder<'_> {
        ImageImportBuilder::new(self.0, source)
    }
    pub async fn inspect(
        &self,
        image: impl AsRef<str>,
    ) -> crate::docker::DockerResult<crate::docker::ImageInspect> {
        let out = self.0.run(["image", "inspect", image.as_ref()]).await?;
        let mut json: Vec<crate::docker::ImageInspect> = serde_json::from_str(&out.stdout)?;
        Ok(json.pop().unwrap_or_default())
    }

    pub async fn inspect_raw(
        &self,
        image: impl AsRef<str>,
    ) -> crate::docker::DockerResult<serde_json::Value> {
        let out = self.0.run(["image", "inspect", image.as_ref()]).await?;
        let mut json: Vec<serde_json::Value> = serde_json::from_str(&out.stdout)?;
        Ok(json.pop().unwrap_or_default())
    }
}

pub mod build;
pub mod lifecycle;
pub mod prune;
pub mod pull;
pub mod query;
pub mod resource;
#[cfg(test)]
pub mod tests;
