use crate::docker::{
    DockerCli, DockerExitStatus, DockerResult, DockerStreamEvent, core::ArgBuilder,
};
use tokio::sync::mpsc;

pub struct SystemEventsBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) args: ArgBuilder,
}

impl<'a> SystemEventsBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["system", "events"]),
        }
    }

    pub fn filter(mut self, f: impl Into<String>) -> Self {
        self.args.pair("--filter", f.into());
        self
    }

    pub fn since(mut self, timestamp: impl Into<String>) -> Self {
        self.args.pair("--since", timestamp.into());
        self
    }

    pub fn until(mut self, timestamp: impl Into<String>) -> Self {
        self.args.pair("--until", timestamp.into());
        self
    }

    pub fn format(mut self, fmt: impl Into<String>) -> Self {
        self.args.pair("--format", fmt.into());
        self
    }

    pub async fn stream(
        self,
        sender: mpsc::Sender<DockerStreamEvent>,
    ) -> DockerResult<DockerExitStatus> {
        self.cli.execute_stream(&self.args, sender).await
    }
}

crate::impl_builder_opts!(SystemEventsBuilder);
