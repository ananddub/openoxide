use crate::utils::docker::{
    DockerCli, DockerExitStatus, DockerOutput, DockerResult, DockerStreamEvent, core::ArgBuilder,
};
use tokio::sync::mpsc;

pub struct ComposeExecBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) args: ArgBuilder,
    pub(crate) service: String,
}

impl<'a> ComposeExecBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, service: impl Into<String>) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["compose", "exec"]),
            service: service.into(),
        }
    }
    pub fn detach(mut self) -> Self {
        self.args.flag("--detach");
        self
    }
    pub fn env(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self {
        self.args
            .pair("--env", format!("{}={}", k.as_ref(), v.as_ref()));
        self
    }
    pub fn user(mut self, u: impl Into<String>) -> Self {
        self.args.pair("--user", u.into());
        self
    }
    pub fn workdir(mut self, w: impl Into<String>) -> Self {
        self.args.pair("--workdir", w.into());
        self
    }
    pub fn privileged(mut self) -> Self {
        self.args.flag("--privileged");
        self
    }
    pub fn index(mut self, i: u32) -> Self {
        self.args.pair("--index", i.to_string());
        self
    }

    pub async fn run(
        mut self,
        cmd: impl IntoIterator<Item = impl Into<String>>,
    ) -> DockerResult<DockerOutput> {
        self.args.push(&self.service);
        self.args.push_all(cmd.into_iter().map(Into::into));
        self.cli.execute(&self.args).await
    }
    pub async fn stream(
        mut self,
        cmd: impl IntoIterator<Item = impl Into<String>>,
        sender: mpsc::Sender<DockerStreamEvent>,
    ) -> DockerResult<DockerExitStatus> {
        self.args.push(&self.service);
        self.args.push_all(cmd.into_iter().map(Into::into));
        self.cli.execute_stream(&self.args, sender).await
    }
    /// Specify an alternate compose file
    pub fn file(mut self, f: impl Into<String>) -> Self {
        self.args.insert_pair(1, "--file", f.into());
        self
    }

    /// Specify an alternate environment file
    pub fn env_file(mut self, f: impl Into<String>) -> Self {
        self.args.insert_pair(1, "--env-file", f.into());
        self
    }

    /// Specify an alternate project name
    pub fn project(mut self, p: impl Into<String>) -> Self {
        self.args.insert_pair(1, "--project-name", p.into());
        self
    }

    /// Specify a profile to enable
    pub fn profile(mut self, p: impl Into<String>) -> Self {
        self.args.insert_pair(1, "--profile", p.into());
        self
    }
}

crate::impl_builder_opts!(ComposeExecBuilder);

impl crate::utils::exec::script::IntoCommand for ComposeExecBuilder<'_> {
    fn build_str(&self) -> String {
        let mut a = self.args.clone();
        a.push(&self.service);
        a.preview()
    }
}
