use crate::utils::docker::{DockerCli, DockerOutput, DockerResult, core::ArgBuilder};

pub struct ComposeConfigBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) args: ArgBuilder,
}

impl<'a> ComposeConfigBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["compose", "config"]),
        }
    }
    pub fn hash(mut self, h: impl Into<String>) -> Self {
        self.args.pair("--hash", h.into());
        self
    }
    pub fn no_interpolate(mut self) -> Self {
        self.args.flag("--no-interpolate");
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
    pub fn volumes(mut self) -> Self {
        self.args.flag("--volumes");
        self
    }

    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }

    pub async fn run_json(mut self) -> DockerResult<serde_json::Value> {
        self.args.pair("--format", "json");
        let output = self.cli.execute(&self.args).await?;
        let json = serde_json::from_str(&output.stdout).map_err(|e| {
            crate::utils::exec::ExecError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                e,
            ))
        })?;
        Ok(json)
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

crate::impl_builder_opts!(ComposeConfigBuilder);

impl crate::utils::exec::script::IntoCommand for ComposeConfigBuilder<'_> {
    fn build_str(&self) -> String {
        self.args.preview()
    }
}
