use crate::utils::docker::{
    DockerCli, DockerExitStatus, DockerResult, DockerStreamEvent, core::ArgBuilder,
};
use tokio::sync::mpsc;

pub struct LogsBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) kind: &'static str,
    pub(crate) id: String,
    pub(crate) args: ArgBuilder,
}

impl<'a> LogsBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self {
        Self {
            cli,
            kind: "container",
            id: id.into(),
            args: ArgBuilder::default(),
        }
    }

    pub fn kind(mut self, kind: &'static str) -> Self {
        self.kind = kind;
        self
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
    pub fn build_command_args(&self) -> Vec<String> {
        let mut a = ArgBuilder::cmd(&[self.kind, "logs"]);
        a.inherit_meta(&self.args);
        a.push_all(self.args.clone().build());
        a.push(&self.id);
        a.build()
    }

    pub fn print(&self) -> String {
        let mut a = ArgBuilder::cmd(&[self.kind, "logs"]);
        a.push_all(self.args.clone().build());
        a.push(&self.id);
        a.preview()
    }

    pub async fn output(self) -> DockerResult<String> {
        let mut a = ArgBuilder::cmd(&[self.kind, "logs"]);
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
        let mut a = ArgBuilder::cmd(&[self.kind, "logs"]);
        a.inherit_meta(&self.args);
        a.push_all(self.args.build());
        a.push(&self.id);
        self.cli.execute_stream(&a, sender).await
    }
}
crate::impl_builder_opts!(LogsBuilder);
