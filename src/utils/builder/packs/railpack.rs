use crate::utils::docker::DockerCli;
use crate::utils::docker::handles::RestartPolicy;
use crate::utils::docker::query::ContainerFilter;
use crate::utils::exec::{ArgBuilder, CommandExecutor, ExecOutput, ExecResult};
use crate::utils::os::OsCli;
use tokio_util::sync::CancellationToken;

#[derive(Clone, Debug)]
pub struct RailpackCli<'a> {
    pub(crate) executor: &'a CommandExecutor,
}

impl<'a> RailpackCli<'a> {
    pub fn new(executor: &'a CommandExecutor) -> Self {
        Self { executor }
    }

    pub async fn exists(&self) -> bool {
        OsCli::new(self.executor)
            .has_command("railpack")
            .run()
            .await
            .map(|out| out.success())
            .unwrap_or(false)
    }

    pub async fn is_exists(&self) -> bool {
        self.exists().await
    }

    pub async fn install(&self) -> ExecResult<ExecOutput> {
        OsCli::new(self.executor)
            .shell_installer("https://railpack.com/install.sh")
            .shell("bash")
            .env("RAILPACK_VERSION", "0.15.4")
            .run()
            .await?;
        self.executor.run("railpack", ["--version"]).await
    }

    pub async fn if_not_exist_install(&self) -> ExecResult<()> {
        if !self.exists().await {
            self.install().await?;
        }
        Ok(())
    }

    pub fn prepare(&self, path: impl Into<String>) -> RailpackPrepareBuilder<'_> {
        let mut args = ArgBuilder::cmd(&["prepare"]);
        args.push(path.into());
        RailpackPrepareBuilder {
            executor: self.executor,
            args,
        }
    }
}

pub struct RailpackPrepareBuilder<'a> {
    executor: &'a CommandExecutor,
    args: ArgBuilder,
}

impl<'a> RailpackPrepareBuilder<'a> {
    pub fn plan_out(mut self, path: impl Into<String>) -> Self {
        self.args.pair("--plan-out", path.into());
        self
    }

    pub fn info_out(mut self, path: impl Into<String>) -> Self {
        self.args.pair("--info-out", path.into());
        self
    }

    pub fn env(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self {
        self.args
            .pair("--env", format!("{}={}", k.as_ref(), v.as_ref()));
        self
    }

    pub fn previous(mut self, prev: impl Into<String>) -> Self {
        self.args.pair("--previous", prev.into());
        self
    }

    pub fn build_cmd(mut self, cmd: impl Into<String>) -> Self {
        self.args.pair("--build-cmd", cmd.into());
        self
    }

    pub fn start_cmd(mut self, cmd: impl Into<String>) -> Self {
        self.args.pair("--start-cmd", cmd.into());
        self
    }

    pub fn config_file(mut self, file: impl Into<String>) -> Self {
        self.args.pair("--config-file", file.into());
        self
    }

    pub fn error_missing_start(mut self) -> Self {
        self.args.flag("--error-missing-start");
        self
    }

    pub async fn run(self, cancel: &CancellationToken) -> ExecResult<ExecOutput> {
        let docker = DockerCli::from_executor(self.executor.clone());
        let name = "rustploy_buildkit".to_string();
        let cointainer = docker
            .containers()
            .ps()
            .filter(ContainerFilter::Name(name.clone()))
            .list()
            .await?;

        if cointainer.is_empty() {
            let _ = docker
                .containers()
                .create("moby/buildkit")
                .name(name.clone())
                .detach()
                .restart(RestartPolicy::Always)
                .privileged()
                .run()
                .await;
        } else {
            let _ = docker.containers().start(name.clone()).run().await;
        }

        self.executor
            .run_cancelled("railpack", self.args.build(), cancel)
            .await
    }

    pub async fn run_in_cgroup(
        self,
        cgroup_path: Option<&str>,
        cancel: &CancellationToken,
    ) -> ExecResult<ExecOutput> {
        let docker = DockerCli::from_executor(self.executor.clone());
        let name = "rustploy_buildkit".to_string();
        let cointainer = docker
            .containers()
            .ps()
            .filter(ContainerFilter::Name(name.clone()))
            .list()
            .await?;

        if cointainer.is_empty() {
            let _ = docker
                .containers()
                .create("moby/buildkit")
                .name(name.clone())
                .detach()
                .restart(RestartPolicy::Always)
                .privileged()
                .run()
                .await;
        } else {
            let _ = docker.containers().start(name.clone()).run().await;
        }

        self.executor
            .run_cancelled_in_cgroup(cgroup_path, "railpack", self.args.build(), cancel)
            .await
    }
}

impl crate::utils::exec::pipeline::IntoCommand for RailpackPrepareBuilder<'_> {
    fn build_str(&self) -> String {
        format!("railpack {}", self.args.build_string())
    }
}
