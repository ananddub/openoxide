use crate::utils::docker::{DockerOutput, DockerResult, client::DockerCli, core::ArgBuilder};

pub struct NodePsBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    node_id: String,
}

impl<'a> NodePsBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, node_id: impl Into<String>) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["node", "ps"]),
            node_id: node_id.into(),
        }
    }

    pub fn filter(mut self, f: crate::utils::docker::query::filter::TaskFilter) -> Self {
        self.args.filter(f);
        self
    }
    pub fn filters(
        mut self,
        fs: impl IntoIterator<Item = crate::utils::docker::query::filter::TaskFilter>,
    ) -> Self {
        for f in fs {
            self.args.filter(f);
        }
        self
    }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        self.args.push(&self.node_id);
        self.cli.execute(&self.args).await
    }

    pub async fn run_json(mut self) -> DockerResult<Vec<crate::utils::docker::TaskSummary>> {
        self.args.pair("--format", "{{json .}}");
        self.args.push(&self.node_id);
        self.cli.execute_json_lines(&self.args).await
    }
}
crate::impl_builder_opts!(NodePsBuilder);
