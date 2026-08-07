use crate::docker::{DockerOutput, DockerResult, client::DockerCli, core::ArgBuilder};

pub struct ServicePsBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    name: String,
}

impl<'a> ServicePsBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, name: impl Into<String>) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["service", "ps"]),
            name: name.into(),
        }
    }

    pub fn filter(mut self, f: crate::docker::query::filter::TaskFilter) -> Self {
        self.args.filter(f);
        self
    }
    pub fn filters(
        mut self,
        fs: impl IntoIterator<Item = crate::docker::query::filter::TaskFilter>,
    ) -> Self {
        for f in fs {
            self.args.filter(f);
        }
        self
    }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        self.args.push(&self.name);
        self.cli.execute(&self.args).await
    }

    pub async fn run_json(mut self) -> DockerResult<Vec<crate::docker::TaskSummary>> {
        self.args.pair("--format", "{{json .}}");
        self.args.push(&self.name);
        self.cli.execute_json_lines(&self.args).await
    }
}
crate::impl_builder_opts!(ServicePsBuilder);

impl crate::exec::script::IntoCommand for ServicePsBuilder<'_> {
    fn build_str(&self) -> String {
        let mut a = self.args.clone();
        a.push(&self.name);
        a.preview()
    }
}
