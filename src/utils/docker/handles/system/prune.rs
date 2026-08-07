use crate::utils::docker::{DockerCli, DockerOutput, DockerResult, core::ArgBuilder};

pub struct SystemPruneBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) args: ArgBuilder,
}

impl<'a> SystemPruneBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["system", "prune", "--force"]),
        }
    }

    pub fn all(mut self, enabled: bool) -> Self {
        if enabled {
            self.args.flag("--all");
        }
        self
    }

    pub fn volumes(mut self, enabled: bool) -> Self {
        if enabled {
            self.args.flag("--volumes");
        }
        self
    }

    pub fn filter(mut self, f: impl Into<String>) -> Self {
        self.args.pair("--filter", f.into());
        self
    }

    pub fn print(&self) -> String {
        self.args.preview()
    }

    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }
}

crate::impl_builder_opts!(SystemPruneBuilder);

impl crate::utils::exec::script::IntoCommand for SystemPruneBuilder<'_> {
    fn build_str(&self) -> String {
        self.args.preview()
    }
}
