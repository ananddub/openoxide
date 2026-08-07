use crate::utils::docker::{
    BuildProgress, DockerCli, DockerExitStatus, DockerOutput, DockerResult, DockerStreamEvent,
    core::ArgBuilder,
};
use tokio::sync::mpsc;

pub struct ComposeBuildBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) args: ArgBuilder,
}

impl<'a> ComposeBuildBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["compose", "build"]),
        }
    }
    pub fn service(mut self, s: impl Into<String>) -> Self {
        self.args.push(s.into());
        self
    }
    pub fn no_cache(mut self) -> Self {
        self.args.flag("--no-cache");
        self
    }
    pub fn pull(mut self) -> Self {
        self.args.flag("--pull");
        self
    }
    pub fn quiet(mut self) -> Self {
        self.args.flag("--quiet");
        self
    }
    pub fn build_arg(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self {
        self.args
            .pair("--build-arg", format!("{}={}", k.as_ref(), v.as_ref()));
        self
    }
    pub fn ssh(mut self, s: impl Into<String>) -> Self {
        self.args.pair("--ssh", s.into());
        self
    }
    pub fn progress(mut self, p: impl Into<BuildProgress>) -> Self {
        let progress: BuildProgress = p.into();
        self.args.pair("--progress", progress.as_str());
        self
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

crate::impl_builder_opts!(ComposeBuildBuilder);

impl crate::utils::exec::script::IntoCommand for ComposeBuildBuilder<'_> {
    fn build_str(&self) -> String {
        self.args.preview()
    }
}
