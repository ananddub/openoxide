use crate::utils::docker::{
    DockerCli, DockerOutput, DockerResult, core::ArgBuilder, query::filter::VolumeFilter,
};

pub struct VolumePrune<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) args: ArgBuilder,
}

impl<'a> VolumePrune<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["volume", "prune", "--force"]),
        }
    }
    pub fn filter(mut self, f: VolumeFilter) -> Self {
        self.args.filter(f);
        self
    }
    pub fn all(mut self) -> Self {
        self.args.flag("--all");
        self
    }
    pub fn print(&self) -> String {
        self.args.preview()
    }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(VolumePrune);

pub struct VolumeRmBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) name: String,
    pub(crate) args: ArgBuilder,
}

impl<'a> VolumeRmBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, name: impl Into<String>) -> Self {
        Self {
            cli,
            name: name.into(),
            args: ArgBuilder::cmd(&["volume", "rm"]),
        }
    }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let mut a = self.args;
        a.push(&self.name);
        self.cli.execute(&a).await
    }
}
crate::impl_builder_opts!(VolumeRmBuilder);
