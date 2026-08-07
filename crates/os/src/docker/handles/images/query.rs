use crate::docker::{
    DockerCli, DockerResult, ImageSummary, core::ArgBuilder, query::filter::ImageFilter,
};

pub struct ImageQuery<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) args: ArgBuilder,
}

impl<'a> ImageQuery<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["image", "ls", "--format", "{{json .}}"]),
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
    pub async fn list(self) -> DockerResult<Vec<ImageSummary>> {
        self.cli.execute_json_lines(&self.args).await
    }
    pub async fn exists(self) -> DockerResult<bool> {
        Ok(!self.list().await?.is_empty())
    }
}
crate::impl_builder_opts!(ImageQuery);
