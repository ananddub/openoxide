use crate::exec::CommandExecutor;
use crate::exec::script::IntoCommand;

pub mod action;
pub mod builder;
pub mod command;

pub use action::MountActionBuilder;
pub use builder::MountBuilder;
pub use command::MountCommandBuilder;

pub struct MountCli<'a> {
    pub(crate) executor: &'a CommandExecutor,
}

impl<'a> MountCli<'a> {
    pub fn list(&self) -> MountCommandBuilder<'a> {
        MountCommandBuilder::new(self.executor, vec![])
    }

    pub fn mount(&self, source: impl IntoCommand, target: impl IntoCommand) -> MountBuilder<'a> {
        MountBuilder::new(self.executor, Some(source), target)
    }

    pub fn mount_ref(&self, target: impl IntoCommand) -> MountBuilder<'a> {
        MountBuilder::new(self.executor, None::<&str>, target)
    }
}
