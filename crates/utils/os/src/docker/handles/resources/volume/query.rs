use crate::docker::{
    DockerCli, DockerResult, VolumeSummary, core::ArgBuilder, query::filter::VolumeFilter,
};

pub struct VolumeQuery<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) args: ArgBuilder,
}

impl<'a> VolumeQuery<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["volume", "ls", "--format", "{{json .}}"]),
        }
    }
    pub fn filter(mut self, f: VolumeFilter) -> Self {
        self.args.filter(f);
        self
    }
    pub fn print(&self) -> String {
        self.args.preview()
    }
    pub async fn list(self) -> DockerResult<Vec<VolumeSummary>> {
        self.cli.execute_json_lines(&self.args).await
    }
    pub async fn exists(self) -> DockerResult<bool> {
        Ok(!self.list().await?.is_empty())
    }
}
crate::impl_builder_opts!(VolumeQuery);
