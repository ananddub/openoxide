use crate::utils::docker::{
    DockerCli, DockerExitStatus, DockerResult, DockerStreamEvent, core::ArgBuilder,
};
use tokio::sync::mpsc;

pub struct LogsBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) id: String,
    pub(crate) args: ArgBuilder,
}

impl<'a> LogsBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self {
        Self {
            cli,
            id: id.into(),
            args: ArgBuilder::default(),
        }
    }
    pub fn follow(mut self) -> Self {
        self.args.flag("--follow");
        self
    }
    pub fn timestamps(mut self) -> Self {
        self.args.flag("--timestamps");
        self
    }
    pub fn tail(mut self, n: usize) -> Self {
        self.args.pair("--tail", n.to_string());
        self
    }
    pub fn since(mut self, v: impl Into<String>) -> Self {
        self.args.pair("--since", v.into());
        self
    }
    pub fn until(mut self, v: impl Into<String>) -> Self {
        self.args.pair("--until", v.into());
        self
    }
    pub fn stdout(mut self) -> Self {
        self.args.flag("--stdout");
        self
    }
    pub fn stderr(mut self) -> Self {
        self.args.flag("--stderr");
        self
    }

    pub fn print(&self) -> String {
        let mut a = ArgBuilder::cmd(&["container", "logs"]);
        a.push_all(self.args.clone().build());
        a.push(&self.id);
        a.preview()
    }

    pub async fn output(self) -> DockerResult<String> {
        let mut a = ArgBuilder::cmd(&["container", "logs"]);
        a.inherit_meta(&self.args);
        a.push_all(self.args.build());
        a.push(&self.id);
        let out = self.cli.execute(&a).await?;
        Ok(format!("{}{}", out.stdout, out.stderr))
    }

    pub async fn stream(
        self,
        sender: mpsc::Sender<DockerStreamEvent>,
    ) -> DockerResult<DockerExitStatus> {
        let mut a = ArgBuilder::cmd(&["container", "logs"]);
        a.inherit_meta(&self.args);
        a.push_all(self.args.build());
        a.push(&self.id);
        self.cli.execute_stream(&a, sender).await
    }
}
crate::impl_builder_opts!(LogsBuilder);
