use crate::docker::{
    DockerCli, DockerOutput, DockerResult, core::ArgBuilder, query::filter::ContainerFilter,
};

pub struct ContainerPrune<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) args: ArgBuilder,
}

impl<'a> ContainerPrune<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["container", "prune", "--force"]),
        }
    }
    pub fn filter(mut self, f: ContainerFilter) -> Self {
        self.args.filter(f);
        self
    }
    pub fn print(&self) -> String {
        self.args.preview()
    }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(ContainerPrune);
