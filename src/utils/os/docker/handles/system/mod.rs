use crate::utils::docker::{DockerCli, DockerResult, DockerVersion};

pub use df::SystemDfBuilder;
pub use events::SystemEventsBuilder;
pub use info::SystemInfoBuilder;
pub use login::SystemLoginBuilder;
pub use logout::SystemLogoutBuilder;
pub use prune::SystemPruneBuilder;

pub mod df;
pub mod events;
pub mod info;
pub mod login;
pub mod logout;
pub mod prune;

pub struct SystemHandle<'a>(pub(crate) &'a DockerCli);

impl<'a> SystemHandle<'a> {
    pub fn info(&self) -> SystemInfoBuilder<'a> {
        SystemInfoBuilder::new(self.0)
    }

    pub async fn version(&self) -> DockerResult<DockerVersion> {
        self.0.json(&["version", "--format", "{{json .}}"]).await
    }

    pub fn df(&self) -> SystemDfBuilder<'a> {
        SystemDfBuilder::new(self.0)
    }

    pub fn events(&self) -> SystemEventsBuilder<'a> {
        SystemEventsBuilder::new(self.0)
    }

    pub fn prune(&self) -> SystemPruneBuilder<'a> {
        SystemPruneBuilder::new(self.0)
    }

    pub fn login(&self) -> SystemLoginBuilder<'a> {
        SystemLoginBuilder::new(self.0)
    }

    pub fn logout(&self) -> SystemLogoutBuilder<'a> {
        SystemLogoutBuilder::new(self.0)
    }
}
