use crate::utils::docker::{
    DockerCli, DockerOutput, DockerResult,
    core::{ArgBuilder, Cpu, Memory},
    handles::containers::query::RestartPolicy,
};

// Start
pub struct ContainerStartBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) id: String,
    pub(crate) args: ArgBuilder,
}

impl<'a> ContainerStartBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self {
        Self {
            cli,
            id: id.into(),
            args: ArgBuilder::cmd(&["container", "start"]),
        }
    }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let mut a = self.args;
        a.push(&self.id);
        self.cli.execute(&a).await
    }
}
crate::impl_builder_opts!(ContainerStartBuilder);

// Stop
pub struct ContainerStopBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) id: String,
    pub(crate) args: ArgBuilder,
}

impl<'a> ContainerStopBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self {
        Self {
            cli,
            id: id.into(),
            args: ArgBuilder::cmd(&["container", "stop"]),
        }
    }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let mut a = self.args;
        a.push(&self.id);
        self.cli.execute(&a).await
    }
}
crate::impl_builder_opts!(ContainerStopBuilder);

// Restart
pub struct ContainerRestartBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) id: String,
    pub(crate) args: ArgBuilder,
}

impl<'a> ContainerRestartBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self {
        Self {
            cli,
            id: id.into(),
            args: ArgBuilder::cmd(&["container", "restart"]),
        }
    }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let mut a = self.args;
        a.push(&self.id);
        self.cli.execute(&a).await
    }
}
crate::impl_builder_opts!(ContainerRestartBuilder);

// Pause
pub struct ContainerPauseBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) id: String,
    pub(crate) args: ArgBuilder,
}

impl<'a> ContainerPauseBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self {
        Self {
            cli,
            id: id.into(),
            args: ArgBuilder::cmd(&["container", "pause"]),
        }
    }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let mut a = self.args;
        a.push(&self.id);
        self.cli.execute(&a).await
    }
}
crate::impl_builder_opts!(ContainerPauseBuilder);

// Unpause
pub struct ContainerUnpauseBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) id: String,
    pub(crate) args: ArgBuilder,
}

impl<'a> ContainerUnpauseBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self {
        Self {
            cli,
            id: id.into(),
            args: ArgBuilder::cmd(&["container", "unpause"]),
        }
    }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let mut a = self.args;
        a.push(&self.id);
        self.cli.execute(&a).await
    }
}
crate::impl_builder_opts!(ContainerUnpauseBuilder);

// Kill
pub struct ContainerKillBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) id: String,
    pub(crate) args: ArgBuilder,
}

impl<'a> ContainerKillBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self {
        Self {
            cli,
            id: id.into(),
            args: ArgBuilder::cmd(&["container", "kill"]),
        }
    }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let mut a = self.args;
        a.push(&self.id);
        self.cli.execute(&a).await
    }
}
crate::impl_builder_opts!(ContainerKillBuilder);

// Wait
pub struct ContainerWaitBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) id: String,
    pub(crate) args: ArgBuilder,
}

impl<'a> ContainerWaitBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self {
        Self {
            cli,
            id: id.into(),
            args: ArgBuilder::cmd(&["container", "wait"]),
        }
    }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let mut a = self.args;
        a.push(&self.id);
        self.cli.execute(&a).await
    }
}
crate::impl_builder_opts!(ContainerWaitBuilder);

// Port
pub struct ContainerPortBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) id: String,
    pub(crate) args: ArgBuilder,
}

impl<'a> ContainerPortBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self {
        Self {
            cli,
            id: id.into(),
            args: ArgBuilder::cmd(&["container", "port"]),
        }
    }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let mut a = self.args;
        a.push(&self.id);
        self.cli.execute(&a).await
    }
}
crate::impl_builder_opts!(ContainerPortBuilder);

// Top
pub struct ContainerTopBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) id: String,
    pub(crate) args: ArgBuilder,
}

impl<'a> ContainerTopBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self {
        Self {
            cli,
            id: id.into(),
            args: ArgBuilder::cmd(&["container", "top"]),
        }
    }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let mut a = self.args;
        a.push(&self.id);
        self.cli.execute(&a).await
    }
}
crate::impl_builder_opts!(ContainerTopBuilder);

// Rm
pub struct ContainerRmBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) id: String,
    pub(crate) args: ArgBuilder,
}

impl<'a> ContainerRmBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self {
        Self {
            cli,
            id: id.into(),
            args: ArgBuilder::cmd(&["container", "rm"]),
        }
    }
    pub fn force(mut self) -> Self {
        self.args.flag("--force");
        self
    }
    pub fn volumes(mut self) -> Self {
        self.args.flag("--volumes");
        self
    }
    pub fn link(mut self) -> Self {
        self.args.flag("--link");
        self
    }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let mut a = self.args;
        a.push(&self.id);
        self.cli.execute(&a).await
    }
}
crate::impl_builder_opts!(ContainerRmBuilder);

// Rename
pub struct ContainerRenameBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) args: ArgBuilder,
}

impl<'a> ContainerRenameBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            cli,
            id: id.into(),
            name: name.into(),
            args: ArgBuilder::cmd(&["container", "rename"]),
        }
    }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let mut a = self.args;
        a.push(&self.id);
        a.push(&self.name);
        self.cli.execute(&a).await
    }
}
crate::impl_builder_opts!(ContainerRenameBuilder);

// Update
pub struct ContainerUpdateBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) id: String,
    pub(crate) args: ArgBuilder,
}

impl<'a> ContainerUpdateBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, id: impl Into<String>) -> Self {
        Self {
            cli,
            id: id.into(),
            args: ArgBuilder::cmd(&["container", "update"]),
        }
    }
    pub fn memory(mut self, m: Memory) -> Self {
        self.args.pair("--memory", m.to_string());
        self
    }
    pub fn memory_str(mut self, m: impl Into<String>) -> Self {
        self.args.pair("--memory", m.into());
        self
    }
    pub fn cpus(mut self, c: Cpu) -> Self {
        self.args.pair("--cpus", c.to_string());
        self
    }
    pub fn cpus_str(mut self, c: impl Into<String>) -> Self {
        self.args.pair("--cpus", c.into());
        self
    }
    pub fn restart(mut self, p: RestartPolicy) -> Self {
        self.args.pair("--restart", p.to_string());
        self
    }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let mut a = self.args;
        a.push(&self.id);
        self.cli.execute(&a).await
    }
}
crate::impl_builder_opts!(ContainerUpdateBuilder);

// IntoCommand Impls for lifecycle builders
impl crate::utils::exec::script::IntoCommand for ContainerStartBuilder<'_> {
    fn build_str(&self) -> String {
        let mut a = self.args.clone();
        a.push(&self.id);
        a.preview()
    }
}

impl crate::utils::exec::script::IntoCommand for ContainerStopBuilder<'_> {
    fn build_str(&self) -> String {
        let mut a = self.args.clone();
        a.push(&self.id);
        a.preview()
    }
}

impl crate::utils::exec::script::IntoCommand for ContainerRestartBuilder<'_> {
    fn build_str(&self) -> String {
        let mut a = self.args.clone();
        a.push(&self.id);
        a.preview()
    }
}

impl crate::utils::exec::script::IntoCommand for ContainerRmBuilder<'_> {
    fn build_str(&self) -> String {
        let mut a = self.args.clone();
        a.push(&self.id);
        a.preview()
    }
}
