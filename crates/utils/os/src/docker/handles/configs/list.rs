use crate::docker::{DockerOutput, DockerResult, client::DockerCli, core::ArgBuilder};

pub struct ConfigListBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
}

impl<'a> ConfigListBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["config", "ls"]),
        }
    }

    pub fn filter(mut self, f: crate::docker::query::filter::ConfigFilter) -> Self {
        self.args.filter(f);
        self
    }
    pub fn filters(
        mut self,
        fs: impl IntoIterator<Item = crate::docker::query::filter::ConfigFilter>,
    ) -> Self {
        for f in fs {
            self.args.filter(f);
        }
        self
    }

    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }

    pub async fn run_json(mut self) -> DockerResult<Vec<crate::docker::ConfigSummary>> {
        self.args.pair("--format", "{{json .}}");
        self.cli.execute_json_lines(&self.args).await
    }
}

crate::impl_builder_opts!(ConfigListBuilder);
