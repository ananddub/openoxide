use crate::docker::{
    DockerExitStatus, DockerOutput, DockerResult, DockerStreamEvent, client::DockerCli,
    core::ArgBuilder,
};
use tokio::sync::mpsc;

pub struct ServiceLogsBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    name: String,
}

impl<'a> ServiceLogsBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, name: impl Into<String>) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["service", "logs"]),
            name: name.into(),
        }
    }

    pub fn follow(mut self) -> Self {
        self.args.flag("--follow");
        self
    }
    pub fn tail(mut self, n: impl AsRef<str>) -> Self {
        self.args.pair("--tail", n);
        self
    }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        self.args.push(&self.name);
        self.cli.execute(&self.args).await
    }

    pub async fn stream(
        mut self,
        sender: mpsc::Sender<DockerStreamEvent>,
    ) -> DockerResult<DockerExitStatus> {
        self.args.push(&self.name);
        self.cli.execute_stream(&self.args, sender).await
    }
}
crate::impl_builder_opts!(ServiceLogsBuilder);

impl crate::exec::script::IntoCommand for ServiceLogsBuilder<'_> {
    fn build_str(&self) -> String {
        let mut a = self.args.clone();
        a.push(&self.name);
        a.preview()
    }
}
