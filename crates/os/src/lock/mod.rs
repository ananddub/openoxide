use crate::exec::CommandExecutor;
use crate::exec::script::IntoCommand;

pub mod acquire;
pub mod release;

pub use acquire::LockAcquireBuilder;
pub use release::LockReleaseBuilder;

pub struct LockCli<'a> {
    pub(crate) executor: &'a CommandExecutor,
}

impl<'a> LockCli<'a> {
    pub fn acquire(&self, name: impl IntoCommand) -> LockAcquireBuilder<'a> {
        LockAcquireBuilder::new(self.executor, name)
    }

    pub fn release(&self, name: impl IntoCommand) -> LockReleaseBuilder<'a> {
        LockReleaseBuilder::new(self.executor, name)
    }
}
