use crate::utils::exec::CommandExecutor;
use crate::utils::exec::script::IntoCommand;

pub mod get;
pub mod wait_healthy;

pub use get::HttpGetBuilder;
pub use wait_healthy::HttpWaitHealthyBuilder;

pub struct HttpCli<'a> {
    pub(crate) executor: &'a CommandExecutor,
}

impl<'a> HttpCli<'a> {
    pub fn get(&self, url: impl IntoCommand) -> HttpGetBuilder<'a> {
        HttpGetBuilder::new(self.executor, url)
    }

    pub fn wait_healthy(
        &self,
        url: impl IntoCommand,
        timeout: impl IntoCommand,
    ) -> HttpWaitHealthyBuilder<'a> {
        HttpWaitHealthyBuilder::new(self.executor, url, timeout)
    }
}
