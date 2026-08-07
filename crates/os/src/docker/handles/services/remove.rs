use crate::docker::{DockerOutput, DockerResult, client::DockerCli, core::ArgBuilder};

pub struct ServiceRemoveBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    name: String,
}

impl<'a> ServiceRemoveBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, name: impl Into<String>) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["service", "rm"]),
            name: name.into(),
        }
    }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        self.args.push(&self.name);
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(ServiceRemoveBuilder);

impl crate::exec::script::IntoCommand for ServiceRemoveBuilder<'_> {
    fn build_str(&self) -> String {
        let mut a = self.args.clone();
        a.push(&self.name);
        a.preview()
    }
}
