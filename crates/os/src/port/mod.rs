use crate::exec::CommandExecutor;
use crate::exec::script::IntoCommand;

pub mod check;
pub mod free;

pub use check::PortCheckBuilder;
pub use free::PortFreeBuilder;

pub struct PortCli<'a> {
    pub(crate) executor: &'a CommandExecutor,
}

impl<'a> PortCli<'a> {
    pub fn free(&self, start: impl IntoCommand) -> PortFreeBuilder<'a> {
        PortFreeBuilder::new(self.executor, start)
    }

    pub fn check(&self, port: impl IntoCommand) -> PortCheckBuilder<'a> {
        PortCheckBuilder::new(self.executor, port)
    }
}
