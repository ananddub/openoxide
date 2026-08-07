use crate::docker::{
    DockerOutput, DockerResult, client::DockerCli, core::ArgBuilder, query::filter::ServiceFilter,
};

pub struct ServiceListBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
}

impl<'a> ServiceListBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["service", "ls"]),
        }
    }

    pub fn filter(mut self, f: ServiceFilter) -> Self {
        self.args.filter(f);
        self
    }
    pub fn filters(mut self, fs: impl IntoIterator<Item = ServiceFilter>) -> Self {
        for f in fs {
            self.args.filter(f);
        }
        self
    }

    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }

    pub async fn run_json(mut self) -> DockerResult<Vec<crate::docker::ServiceSummary>> {
        self.args.pair("--format", "{{json .}}");
        self.cli.execute_json_lines(&self.args).await
    }
}
crate::impl_builder_opts!(ServiceListBuilder);

impl crate::exec::script::IntoCommand for ServiceListBuilder<'_> {
    fn build_str(&self) -> String {
        self.args.preview()
    }
}
