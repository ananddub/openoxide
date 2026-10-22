use crate::exec::CommandExecutor;
use crate::exec::script::IntoCommand;

pub mod all;
pub mod exists;
pub mod get;
pub mod set;
pub mod unset;

pub use all::EnvAllBuilder;
pub use exists::EnvExistsBuilder;
pub use get::EnvGetBuilder;
pub use set::EnvSetBuilder;
pub use unset::EnvUnsetBuilder;

pub struct EnvCli<'a> {
    pub(crate) executor: &'a CommandExecutor,
}

impl<'a> EnvCli<'a> {
    pub fn get(&self, key: impl IntoCommand) -> EnvGetBuilder<'a> {
        EnvGetBuilder::new(self.executor, key)
    }
    pub fn set(&self, key: impl IntoCommand, val: impl IntoCommand) -> EnvSetBuilder<'a> {
        EnvSetBuilder::new(self.executor, key, val)
    }
    pub fn unset(&self, key: impl IntoCommand) -> EnvUnsetBuilder<'a> {
        EnvUnsetBuilder::new(self.executor, key)
    }
    pub fn exists(&self, key: impl IntoCommand) -> EnvExistsBuilder<'a> {
        EnvExistsBuilder::new(self.executor, key)
    }
    pub fn all(&self) -> EnvAllBuilder<'a> {
        EnvAllBuilder::new(self.executor)
    }
}
