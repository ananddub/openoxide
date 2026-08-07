use crate::utils::docker::{
    ComposeContainer, DockerCli, DockerOutput, DockerResult, core::ArgBuilder,
};

pub struct ComposePsBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) args: ArgBuilder,
}

impl<'a> ComposePsBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["compose", "ps"]),
        }
    }
    pub fn all(mut self) -> Self {
        self.args.flag("--all");
        self
    }
    pub fn service(mut self, s: impl Into<String>) -> Self {
        self.args.push(s.into());
        self
    }
    pub fn filter(mut self, f: impl Into<String>) -> Self {
        self.args.pair("--filter", f.into());
        self
    }
    pub fn status(mut self, s: impl Into<String>) -> Self {
        self.args.pair("--status", s.into());
        self
    }
    pub fn quiet(mut self) -> Self {
        self.args.flag("--quiet");
        self
    }
    pub fn services(mut self) -> Self {
        self.args.flag("--services");
        self
    }
    pub fn print(&self) -> String {
        self.args.preview()
    }

    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }

    pub async fn list(mut self) -> DockerResult<Vec<ComposeContainer>> {
        self.args.pair("--format", "json");
        self.cli.execute_json_lines(&self.args).await
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

crate::impl_builder_opts!(ComposePsBuilder);

impl crate::utils::exec::script::IntoCommand for ComposePsBuilder<'_> {
    fn build_str(&self) -> String {
        self.args.preview()
    }
}
