use crate::docker::{
    DockerCli, DockerOutput, DockerResult, core::ArgBuilder, query::filter::ImageFilter,
};

pub struct ImagePrune<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) args: ArgBuilder,
}

impl<'a> ImagePrune<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["image", "prune", "--force"]),
        }
    }
    pub fn all(mut self) -> Self {
        self.args.flag("--all");
        self
    }
    pub fn filter(mut self, f: ImageFilter) -> Self {
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
crate::impl_builder_opts!(ImagePrune);
