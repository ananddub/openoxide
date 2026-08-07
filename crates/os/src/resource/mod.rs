use crate::exec::CommandExecutor;

pub mod command;

pub use command::ResourceCommandBuilder;

pub struct ResourceCli<'a> {
    pub(crate) executor: &'a CommandExecutor,
}

impl<'a> ResourceCli<'a> {
    pub fn memory_usage(&self) -> ResourceCommandBuilder<'a> {
        ResourceCommandBuilder::new(self.executor, "free", vec!["-h".to_string()])
    }
}
