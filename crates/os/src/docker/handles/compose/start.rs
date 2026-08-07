use crate::docker::{DockerCli, DockerOutput, DockerResult, core::ArgBuilder};

pub struct ComposeStartBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) args: ArgBuilder,
}

impl<'a> ComposeStartBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["compose", "start"]),
        }
    }
    pub fn service(mut self, s: impl Into<String>) -> Self {
        self.args.push(s.into());
        self
    }

    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
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

crate::impl_builder_opts!(ComposeStartBuilder);

impl crate::exec::script::IntoCommand for ComposeStartBuilder<'_> {
    fn build_str(&self) -> String {
        self.args.preview()
    }
}
