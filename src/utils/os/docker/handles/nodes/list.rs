use crate::utils::docker::{DockerOutput, DockerResult, client::DockerCli, core::ArgBuilder};

pub struct NodeListBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
}

impl<'a> NodeListBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["node", "ls"]),
        }
    }

    pub fn filter(mut self, f: crate::utils::docker::query::filter::NodeFilter) -> Self {
        self.args.filter(f);
        self
    }
    pub fn filters(
        mut self,
        fs: impl IntoIterator<Item = crate::utils::docker::query::filter::NodeFilter>,
    ) -> Self {
        for f in fs {
            self.args.filter(f);
        }
        self
    }

    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }

    pub async fn run_json(mut self) -> DockerResult<Vec<crate::utils::docker::NodeSummary>> {
        self.args.pair("--format", "{{json .}}");
        self.cli.execute_json_lines(&self.args).await
    }
}
crate::impl_builder_opts!(NodeListBuilder);
