use crate::utils::docker::{DockerOutput, DockerResult, client::DockerCli, core::ArgBuilder};

pub struct ServiceCreateBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    image: String,
}

impl<'a> ServiceCreateBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, image: impl Into<String>) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["service", "create"]),
            image: image.into(),
        }
    }

    pub fn name(mut self, v: impl AsRef<str>) -> Self {
        self.args.pair("--name", v);
        self
    }
    pub fn replicas(mut self, n: u32) -> Self {
        self.args.pair("--replicas", n.to_string());
        self
    }
    pub fn env(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self {
        self.args
            .pair("--env", format!("{}={}", k.as_ref(), v.as_ref()));
        self
    }
    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        self.args.push(&self.image);
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(ServiceCreateBuilder);

impl crate::utils::exec::script::IntoCommand for ServiceCreateBuilder<'_> {
    fn build_str(&self) -> String {
        let mut a = self.args.clone();
        a.push(&self.image);
        a.preview()
    }
}
