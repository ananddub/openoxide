use crate::utils::docker::{
    DockerCli, DockerExitStatus, DockerResult, DockerStreamEvent, core::ArgBuilder,
};
use tokio::sync::mpsc;

pub struct StatsBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) id: String,
    pub(crate) args: ArgBuilder,
}

impl<'a> StatsBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self {
        Self {
            cli,
            id: id.into(),
            args: ArgBuilder::default(),
        }
    }
    pub fn no_stream(mut self) -> Self {
        self.args.flag("--no-stream");
        self
    }

    pub fn build_command_args(&self) -> Vec<String> {
        let mut a = ArgBuilder::cmd(&["container", "stats", "--format", "{{json .}}"]);
        a.inherit_meta(&self.args);
        a.push_all(self.args.clone().build());
        a.push(&self.id);
        a.build()
    }

    pub async fn stream(
        self,
        sender: mpsc::Sender<DockerStreamEvent>,
    ) -> DockerResult<DockerExitStatus> {
        let mut a = ArgBuilder::cmd(&["container", "stats", "--format", "{{json .}}"]);
        a.inherit_meta(&self.args);
        a.push_all(self.args.build());
        a.push(&self.id);
        self.cli.execute_stream(&a, sender).await
    }
}
crate::impl_builder_opts!(StatsBuilder);
