use crate::utils::docker::{DockerOutput, DockerResult, client::DockerCli, core::ArgBuilder};

// Update
pub struct ServiceUpdateBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    name: String,
}

impl<'a> ServiceUpdateBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, name: impl Into<String>) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["service", "update"]),
            name: name.into(),
        }
    }

    pub fn image(mut self, v: impl AsRef<str>) -> Self {
        self.args.pair("--image", v);
        self
    }
    pub fn replicas(mut self, n: u32) -> Self {
        self.args.pair("--replicas", n.to_string());
        self
    }
    pub fn force(mut self) -> Self {
        self.args.flag("--force");
        self
    }
    pub fn label_add(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self {
        self.args
            .pair("--label-add", format!("{}={}", k.as_ref(), v.as_ref()));
        self
    }
    pub fn label_rm(mut self, k: impl AsRef<str>) -> Self {
        self.args.pair("--label-rm", k);
        self
    }
    pub fn env_add(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self {
        self.args
            .pair("--env-add", format!("{}={}", k.as_ref(), v.as_ref()));
        self
    }
    pub fn env_rm(mut self, k: impl AsRef<str>) -> Self {
        self.args.pair("--env-rm", k);
        self
    }
    pub fn limit_memory(mut self, v: impl AsRef<str>) -> Self {
        self.args.pair("--limit-memory", v);
        self
    }
    pub fn update_parallelism(mut self, n: u32) -> Self {
        self.args.pair("--update-parallelism", n.to_string());
        self
    }
    pub fn update_delay(mut self, v: impl AsRef<str>) -> Self {
        self.args.pair("--update-delay", v);
        self
    }
    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        self.args.push(&self.name);
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(ServiceUpdateBuilder);

// Scale
pub struct ServiceScaleBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
}

impl<'a> ServiceScaleBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["service", "scale"]),
        }
    }

    pub fn service(mut self, name: impl AsRef<str>, replicas: u32) -> Self {
        self.args.push(format!("{}={}", name.as_ref(), replicas));
        self
    }

    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(ServiceScaleBuilder);

// Rollback
pub struct ServiceRollbackBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    name: String,
}

impl<'a> ServiceRollbackBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, name: impl Into<String>) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["service", "rollback"]),
            name: name.into(),
        }
    }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        self.args.push(&self.name);
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(ServiceRollbackBuilder);

// IntoCommand Impls
impl crate::utils::exec::script::IntoCommand for ServiceUpdateBuilder<'_> {
    fn build_str(&self) -> String {
        let mut a = self.args.clone();
        a.push(&self.name);
        a.preview()
    }
}

impl crate::utils::exec::script::IntoCommand for ServiceScaleBuilder<'_> {
    fn build_str(&self) -> String {
        self.args.preview()
    }
}

impl crate::utils::exec::script::IntoCommand for ServiceRollbackBuilder<'_> {
    fn build_str(&self) -> String {
        let mut a = self.args.clone();
        a.push(&self.name);
        a.preview()
    }
}
