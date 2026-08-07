use crate::utils::docker::{
    DockerCli, DockerExitStatus, DockerOutput, DockerResult, DockerStreamEvent, core::ArgBuilder,
};
use tokio::sync::mpsc;

pub struct UpBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) args: ArgBuilder,
}

impl<'a> UpBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["compose", "up"]),
        }
    }
    pub fn service(mut self, s: impl Into<String>) -> Self {
        self.args.push(s.into());
        self
    }
    pub fn detach(mut self) -> Self {
        self.args.flag("--detach");
        self
    }
    pub fn build(mut self) -> Self {
        self.args.flag("--build");
        self
    }
    pub fn no_build(mut self) -> Self {
        self.args.flag("--no-build");
        self
    }
    pub fn no_deps(mut self) -> Self {
        self.args.flag("--no-deps");
        self
    }
    pub fn remove_orphans(mut self) -> Self {
        self.args.flag("--remove-orphans");
        self
    }
    pub fn force_recreate(mut self) -> Self {
        self.args.flag("--force-recreate");
        self
    }
    pub fn no_recreate(mut self) -> Self {
        self.args.flag("--no-recreate");
        self
    }
    pub fn no_start(mut self) -> Self {
        self.args.flag("--no-start");
        self
    }
    pub fn quiet_pull(mut self) -> Self {
        self.args.flag("--quiet-pull");
        self
    }
    pub fn abort_on_container_exit(mut self) -> Self {
        self.args.flag("--abort-on-container-exit");
        self
    }
    pub fn wait(mut self) -> Self {
        self.args.flag("--wait");
        self
    }
    pub fn timeout(mut self, t: u32) -> Self {
        self.args.pair("--timeout", t.to_string());
        self
    }
    pub fn scale(mut self, service: impl AsRef<str>, n: u32) -> Self {
        self.args
            .pair("--scale", format!("{}={}", service.as_ref(), n));
        self
    }
    pub fn print(&self) -> String {
        self.args.preview()
    }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }
    pub async fn stream(
        self,
        sender: mpsc::Sender<DockerStreamEvent>,
    ) -> DockerResult<DockerExitStatus> {
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

crate::impl_builder_opts!(UpBuilder);

impl crate::utils::exec::script::IntoCommand for UpBuilder<'_> {
    fn build_str(&self) -> String {
        self.args.preview()
    }
}
