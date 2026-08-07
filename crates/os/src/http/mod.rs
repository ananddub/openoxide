use crate::exec::CommandExecutor;
use crate::exec::script::IntoCommand;

pub mod get;
pub mod wait_healthy;

pub use get::HttpGetBuilder;
pub use wait_healthy::HttpWaitHealthyBuilder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Head,
}

impl HttpMethod {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Head => "HEAD",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthyStatus {
    Success,
    SuccessOrRedirect,
}

impl HealthyStatus {
    pub(crate) const fn pattern(self) -> &'static str {
        match self {
            Self::Success => "^2",
            Self::SuccessOrRedirect => "^(2|3)",
        }
    }
}

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
