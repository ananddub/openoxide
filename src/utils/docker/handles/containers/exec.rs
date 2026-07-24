use crate::utils::docker::{
    DockerCli, DockerExitStatus, DockerOutput, DockerResult, DockerStreamEvent, core::ArgBuilder,
};
use tokio::sync::mpsc;

pub struct ExecBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) id: String,
    pub(crate) args: ArgBuilder,
}

impl<'a> ExecBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self {
        Self {
            cli,
            id: id.into(),
            args: ArgBuilder::default(),
        }
    }
    pub fn user(mut self, v: impl Into<String>) -> Self {
        self.args.pair("--user", v.into());
        self
    }
    pub fn workdir(mut self, v: impl Into<String>) -> Self {
        self.args.pair("--workdir", v.into());
        self
    }
    pub fn env(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self {
        self.args.env_var(k, v);
        self
    }
    pub fn tty(mut self, enabled: bool) -> Self {
        self.args.flag_if("--tty", enabled);
        self
    }
    pub fn interactive(mut self) -> Self {
        self.args.flag("--interactive");
        self
    }
    pub fn detach(mut self) -> Self {
        self.args.flag("--detach");
        self
    }

    pub fn print(&self, cmd: &[impl AsRef<str>]) -> String {
        let mut a = ArgBuilder::cmd(&["container", "exec"]);
        a.push_all(self.args.clone().build());
        a.push(&self.id);
        a.push_all(cmd.iter().map(|s| s.as_ref().to_string()));
        a.preview()
    }

    pub async fn run(
        self,
        cmd: impl IntoIterator<Item = impl Into<String>>,
    ) -> DockerResult<DockerOutput> {
        let mut a = ArgBuilder::cmd(&["container", "exec"]);
        a.inherit_meta(&self.args);
        a.push_all(self.args.build());
        a.push(&self.id);
        a.push_all(cmd.into_iter().map(Into::into));
        self.cli.execute(&a).await
    }

    pub async fn run_stream(
        self,
        cmd: impl IntoIterator<Item = impl Into<String>>,
        sender: mpsc::Sender<DockerStreamEvent>,
    ) -> DockerResult<DockerExitStatus> {
        let mut a = ArgBuilder::cmd(&["container", "exec"]);
        a.inherit_meta(&self.args);
        a.push_all(self.args.build());
        a.push(&self.id);
        a.push_all(cmd.into_iter().map(Into::into));
        self.cli.execute_stream(&a, sender).await
    }

    pub fn build_args(self, cmd: impl IntoIterator<Item = impl Into<String>>) -> Vec<String> {
        let mut a = ArgBuilder::cmd(&["exec"]);
        a.inherit_meta(&self.args);
        a.push_all(self.args.build());
        a.push(&self.id);
        a.push_all(cmd.into_iter().map(Into::into));
        a.build()
    }
}
crate::impl_builder_opts!(ExecBuilder);
