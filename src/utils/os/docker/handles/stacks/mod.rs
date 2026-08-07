use crate::utils::docker::client::DockerCli;

pub use config::StackConfigBuilder;
pub use deploy::StackDeployBuilder;
pub use list::StackListBuilder;
pub use ps::StackPsBuilder;
pub use remove::StackRemoveBuilder;
pub use services::StackServicesBuilder;

// ── StacksHandle ────────────────────────────────────────────────────────────

pub struct StacksHandle<'a> {
    cli: &'a DockerCli,
}

impl<'a> StacksHandle<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self { cli }
    }

    pub fn deploy(&self, stack_name: impl Into<String>) -> StackDeployBuilder<'a> {
        StackDeployBuilder::new(self.cli, stack_name)
    }

    pub fn remove(&self, stack_name: impl Into<String>) -> StackRemoveBuilder<'a> {
        StackRemoveBuilder::new(self.cli, stack_name)
    }

    pub fn list(&self) -> StackListBuilder<'a> {
        StackListBuilder::new(self.cli)
    }

    pub fn ps(&self, stack_name: impl Into<String>) -> StackPsBuilder<'a> {
        StackPsBuilder::new(self.cli, stack_name)
    }

    pub fn services(&self, stack_name: impl Into<String>) -> StackServicesBuilder<'a> {
        StackServicesBuilder::new(self.cli, stack_name)
    }

    pub fn config(&self, stack_name: impl Into<String>) -> StackConfigBuilder<'a> {
        StackConfigBuilder::new(self.cli, stack_name)
    }
}

pub mod config;
pub mod deploy;
pub mod list;
pub mod ps;
pub mod remove;
pub mod services;
