use crate::docker::{DockerOutput, DockerResult, client::DockerCli, core::ArgBuilder};

pub struct StackServicesBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    stack_name: String,
}

impl<'a> StackServicesBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, stack_name: impl Into<String>) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["stack", "services"]),
            stack_name: stack_name.into(),
        }
    }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        self.args.push(&self.stack_name);
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(StackServicesBuilder);

impl crate::exec::script::IntoCommand for StackServicesBuilder<'_> {
    fn build_str(&self) -> String {
        let mut a = self.args.clone();
        a.push(&self.stack_name);
        a.preview()
    }
}
