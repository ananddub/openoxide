use crate::utils::docker::{DockerOutput, DockerResult, client::DockerCli, core::ArgBuilder};

pub struct StackConfigBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    stack_name: String,
}

impl<'a> StackConfigBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, stack_name: impl Into<String>) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["stack", "config"]),
            stack_name: stack_name.into(),
        }
    }

    pub fn compose_file(mut self, path: impl AsRef<str>) -> Self {
        self.args.pair("--compose-file", path);
        self
    }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        self.args.push(&self.stack_name);
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(StackConfigBuilder);

impl crate::utils::exec::script::IntoCommand for StackConfigBuilder<'_> {
    fn build_str(&self) -> String {
        let mut a = self.args.clone();
        a.push(&self.stack_name);
        a.preview()
    }
}
