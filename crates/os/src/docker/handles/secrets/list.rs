use crate::docker::{DockerOutput, DockerResult, client::DockerCli, core::ArgBuilder};

pub struct SecretListBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
}

impl<'a> SecretListBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["secret", "ls"]),
        }
    }

    pub fn filter(mut self, f: crate::docker::query::filter::SecretFilter) -> Self {
        self.args.filter(f);
        self
    }
    pub fn filters(
        mut self,
        fs: impl IntoIterator<Item = crate::docker::query::filter::SecretFilter>,
    ) -> Self {
        for f in fs {
            self.args.filter(f);
        }
        self
    }

    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }

    pub async fn run_json(mut self) -> DockerResult<Vec<crate::docker::SecretSummary>> {
        self.args.pair("--format", "{{json .}}");
        self.cli.execute_json_lines(&self.args).await
    }
}
crate::impl_builder_opts!(SecretListBuilder);
