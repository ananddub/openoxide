use crate::utils::exec::CommandExecutor;
use crate::utils::exec::script::IntoCommand;

pub mod wait_healthy;

pub use wait_healthy::HttpWaitHealthyBuilder;

pub struct HttpCli<'a> {
    pub(crate) executor: &'a CommandExecutor,
}

impl<'a> HttpCli<'a> {
    pub fn wait_healthy(
        &self,
        url: impl IntoCommand,
        timeout: impl IntoCommand,
    ) -> HttpWaitHealthyBuilder<'a> {
        HttpWaitHealthyBuilder::new(self.executor, url, timeout)
    }
}
