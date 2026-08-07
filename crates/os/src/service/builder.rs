use super::{ServiceActionBuilder, ServiceLogsBuilder};
use crate::exec::CommandExecutor;
use crate::exec::script::IntoCommand;

pub struct ServiceBuilder<'a> {
    pub(crate) executor: &'a CommandExecutor,
    pub(crate) name: String,
}

impl<'a> ServiceBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, name: impl IntoCommand) -> Self {
        Self {
            executor,
            name: name.build_str(),
        }
    }
    pub fn start(self) -> ServiceActionBuilder<'a> {
        ServiceActionBuilder::new(self.executor, "start", self.name)
    }
    pub fn stop(self) -> ServiceActionBuilder<'a> {
        ServiceActionBuilder::new(self.executor, "stop", self.name)
    }
    pub fn restart(self) -> ServiceActionBuilder<'a> {
        ServiceActionBuilder::new(self.executor, "restart", self.name)
    }
    pub fn reload(self) -> ServiceActionBuilder<'a> {
        ServiceActionBuilder::new(self.executor, "reload", self.name)
    }
    pub fn status(self) -> ServiceActionBuilder<'a> {
        ServiceActionBuilder::new(self.executor, "status", self.name)
    }
    pub fn enabled(self) -> ServiceActionBuilder<'a> {
        ServiceActionBuilder::new(self.executor, "is-enabled", self.name)
    }
    pub fn running(self) -> ServiceActionBuilder<'a> {
        ServiceActionBuilder::new(self.executor, "is-active", self.name)
    }
    pub fn enable(self) -> ServiceActionBuilder<'a> {
        ServiceActionBuilder::new(self.executor, "enable", self.name)
    }
    pub fn disable(self) -> ServiceActionBuilder<'a> {
        ServiceActionBuilder::new(self.executor, "disable", self.name)
    }
    pub fn mask(self) -> ServiceActionBuilder<'a> {
        ServiceActionBuilder::new(self.executor, "mask", self.name)
    }
    pub fn unmask(self) -> ServiceActionBuilder<'a> {
        ServiceActionBuilder::new(self.executor, "unmask", self.name)
    }
    pub fn logs(self, limit: impl IntoCommand) -> ServiceLogsBuilder<'a> {
        ServiceLogsBuilder::new(self.executor, self.name, limit)
    }
}
