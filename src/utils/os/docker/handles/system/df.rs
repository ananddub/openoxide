use crate::utils::docker::{DockerCli, DockerDiskUsage, DockerResult, core::ArgBuilder};

pub struct SystemDfBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) args: ArgBuilder,
}

impl<'a> SystemDfBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["system", "df", "--format", "{{json .}}"]),
        }
    }

    pub fn verbose(mut self) -> Self {
        self.args.flag("--verbose");
        self
    }

    pub fn all(mut self, enabled: bool) -> Self {
        if enabled {
            self.args.flag("--all");
        }
        self
    }

    pub fn format(mut self, fmt: impl Into<String>) -> Self {
        self.args.pair("--format", fmt.into());
        self
    }

    pub async fn run(self) -> DockerResult<DockerDiskUsage> {
        let args = self.args.build();
        let refs: Vec<&str> = args.iter().map(String::as_str).collect();
        self.cli.json(&refs).await
    }
}

crate::impl_builder_opts!(SystemDfBuilder);
