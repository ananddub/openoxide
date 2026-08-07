use crate::docker::{DockerCli, DockerInfo, DockerResult, core::ArgBuilder};

pub struct SystemInfoBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) args: ArgBuilder,
}

impl<'a> SystemInfoBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["info", "--format", "{{json .}}"]),
        }
    }

    pub fn format(mut self, fmt: impl Into<String>) -> Self {
        self.args.pair("--format", fmt.into());
        self
    }

    pub async fn run(self) -> DockerResult<DockerInfo> {
        let args = self.args.build();
        let refs: Vec<&str> = args.iter().map(String::as_str).collect();
        self.cli.json(&refs).await
    }
}

crate::impl_builder_opts!(SystemInfoBuilder);
