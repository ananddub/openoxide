use crate::utils::docker::{
    DockerCli, DockerResult, NetworkSummary, core::ArgBuilder, query::filter::NetworkFilter,
};

pub struct NetworkQuery<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) args: ArgBuilder,
}

impl<'a> NetworkQuery<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["network", "ls", "--format", "{{json .}}"]),
        }
    }
    pub fn filter(mut self, f: NetworkFilter) -> Self {
        self.args.filter(f);
        self
    }
    pub fn print(&self) -> String {
        self.args.preview()
    }
    pub async fn list(self) -> DockerResult<Vec<NetworkSummary>> {
        self.cli.execute_json_lines(&self.args).await
    }
    pub async fn exists(self) -> DockerResult<bool> {
        Ok(!self.list().await?.is_empty())
    }
}
crate::impl_builder_opts!(NetworkQuery);
