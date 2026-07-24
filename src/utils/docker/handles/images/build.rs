use crate::utils::docker::{
    BuildProgress, DockerCli, DockerExitStatus, DockerOutput, DockerResult, DockerStreamEvent,
    core::{ArgBuilder, Platform},
};
use std::path::Path;
use tokio::sync::mpsc;

pub struct BuildBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) context: String,
    pub(crate) args: ArgBuilder,
}

impl<'a> BuildBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, context: impl AsRef<Path>) -> Self {
        Self {
            cli,
            context: context.as_ref().to_string_lossy().into_owned(),
            args: ArgBuilder::default(),
        }
    }
    pub fn dockerfile(mut self, v: impl AsRef<Path>) -> Self {
        self.args.pair("--file", v.as_ref().to_string_lossy());
        self
    }
    pub fn tag(mut self, t: impl Into<String>) -> Self {
        self.args.pair("--tag", t.into());
        self
    }
    pub fn target(mut self, t: impl Into<String>) -> Self {
        self.args.pair("--target", t.into());
        self
    }
    pub fn build_arg(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self {
        self.args
            .pair("--build-arg", format!("{}={}", k.as_ref(), v.as_ref()));
        self
    }
    pub fn cache_from(mut self, v: impl Into<String>) -> Self {
        self.args.pair("--cache-from", v.into());
        self
    }
    pub fn cache_to(mut self, v: impl Into<String>) -> Self {
        self.args.pair("--cache-to", v.into());
        self
    }
    pub fn secret(mut self, v: impl Into<String>) -> Self {
        self.args.pair("--secret", v.into());
        self
    }
    pub fn ssh(mut self, v: impl Into<String>) -> Self {
        self.args.pair("--secret", v.into());
        self
    }
    pub fn no_cache(mut self) -> Self {
        self.args.flag("--no-cache");
        self
    }
    pub fn pull(mut self) -> Self {
        self.args.flag("--pull");
        self
    }
    pub fn platform(mut self, p: Platform) -> Self {
        self.args.pair("--platform", p.to_string());
        self
    }
    pub fn label(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self {
        self.args.label(k, v);
        self
    }
    pub fn progress(mut self, v: impl Into<BuildProgress>) -> Self {
        let p: BuildProgress = v.into();
        self.args.pair("--progress", p.as_str());
        self
    }
    pub fn arg(mut self, v: impl Into<String>) -> Self {
        self.args.push(v.into());
        self
    }

    pub fn print(&self) -> String {
        let mut a = ArgBuilder::cmd(&["image", "build"]);
        a.push_all(self.args.clone().build());
        a.push(&self.context);
        a.preview()
    }

    pub async fn build(self) -> DockerResult<DockerOutput> {
        let mut a = ArgBuilder::cmd(&["image", "build"]);
        a.push_all(self.args.build());
        a.push(&self.context);
        self.cli.execute(&a).await
    }

    pub async fn stream(
        self,
        sender: mpsc::Sender<DockerStreamEvent>,
    ) -> DockerResult<DockerExitStatus> {
        let mut a = ArgBuilder::cmd(&["image", "build"]);
        a.push_all(self.args.build());
        a.push(&self.context);
        self.cli.execute_stream(&a, sender).await
    }
}
crate::impl_builder_opts!(BuildBuilder);

impl crate::utils::exec::script::IntoCommand for BuildBuilder<'_> {
    fn build_str(&self) -> String {
        let mut a = ArgBuilder::cmd(&["image", "build"]);
        a.push_all(self.args.clone().build());
        a.push(&self.context);
        a.preview()
    }
}
