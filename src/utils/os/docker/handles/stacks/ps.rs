use crate::utils::docker::{DockerOutput, DockerResult, client::DockerCli, core::ArgBuilder};

pub struct StackPsBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    stack_name: String,
}

impl<'a> StackPsBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, stack_name: impl Into<String>) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["stack", "ps"]),
            stack_name: stack_name.into(),
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
        self.args.push(&self.stack_name);
        self.cli.execute(&self.args).await
    }

    pub async fn run_json(mut self) -> DockerResult<Vec<crate::utils::docker::TaskSummary>> {
        self.args.pair("--format", "{{json .}}");
        self.args.push(&self.stack_name);
        self.cli.execute_json_lines(&self.args).await
    }
}
crate::impl_builder_opts!(StackPsBuilder);

impl crate::utils::exec::script::IntoCommand for StackPsBuilder<'_> {
    fn build_str(&self) -> String {
        let mut a = self.args.clone();
        a.push(&self.stack_name);
        a.preview()
    }
}
