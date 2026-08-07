use crate::utils::docker::{DockerCli, DockerOutput, DockerResult, RmiMode, core::ArgBuilder};

pub struct DownBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) args: ArgBuilder,
}

impl<'a> DownBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["compose", "down"]),
        }
    }
    pub fn volumes(mut self) -> Self {
        self.args.flag("--volumes");
        self
    }
    pub fn remove_orphans(mut self) -> Self {
        self.args.flag("--remove-orphans");
        self
    }
    pub fn rmi(mut self, kind: impl Into<RmiMode>) -> Self {
        let k: RmiMode = kind.into();
        self.args.pair("--rmi", k.as_str());
        self
    }
    pub fn timeout(mut self, t: u32) -> Self {
        self.args.pair("--timeout", t.to_string());
        self
    }
    pub fn print(&self) -> String {
        self.args.preview()
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

crate::impl_builder_opts!(DownBuilder);

impl crate::utils::exec::script::IntoCommand for DownBuilder<'_> {
    fn build_str(&self) -> String {
        self.args.preview()
    }
}
