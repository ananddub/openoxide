use crate::docker::{
    DockerCli, DockerExitStatus, DockerOutput, DockerResult, DockerStreamEvent,
    core::{ArgBuilder, Platform},
};
use tokio::sync::mpsc;

pub struct PullBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) image: String,
    pub(crate) args: ArgBuilder,
}

impl<'a> PullBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, image: impl Into<String>) -> Self {
        Self {
            cli,
            image: image.into(),
            args: ArgBuilder::default(),
        }
    }
    pub fn platform(mut self, p: Platform) -> Self {
        self.args.pair("--platform", p.to_string());
        self
    }
    pub fn all_tags(mut self) -> Self {
        self.args.flag("--all-tags");
        self
    }
    pub fn print(&self) -> String {
        let mut a = ArgBuilder::cmd(&["image", "pull"]);
        a.push_all(self.args.clone().build());
        a.push(&self.image);
        a.preview()
    }
    pub async fn pull(self) -> DockerResult<DockerOutput> {
        let mut a = ArgBuilder::cmd(&["image", "pull"]);
        a.push_all(self.args.build());
        a.push(&self.image);
        self.cli.execute(&a).await
    }
    pub async fn stream(
        self,
        sender: mpsc::Sender<DockerStreamEvent>,
    ) -> DockerResult<DockerExitStatus> {
        let mut a = ArgBuilder::cmd(&["image", "pull"]);
        a.push_all(self.args.build());
        a.push(&self.image);
        self.cli.execute_stream(&a, sender).await
    }
}
crate::impl_builder_opts!(PullBuilder);

impl crate::exec::script::IntoCommand for PullBuilder<'_> {
    fn build_str(&self) -> String {
        let mut a = ArgBuilder::cmd(&["image", "pull"]);
        a.push_all(self.args.clone().build());
        a.push(&self.image);
        a.preview()
    }
}
