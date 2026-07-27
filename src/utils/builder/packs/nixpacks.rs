use crate::utils::exec::{ArgBuilder, CommandExecutor, ExecOutput, ExecResult};
use crate::utils::os::OsCli;
use tokio_util::sync::CancellationToken;

#[derive(Clone, Debug)]
pub struct NixpacksCli<'a> {
    pub(crate) executor: &'a CommandExecutor,
}

impl<'a> NixpacksCli<'a> {
    pub fn new(executor: &'a CommandExecutor) -> Self {
        Self { executor }
    }

    pub async fn exists(&self) -> bool {
        OsCli::new(self.executor)
            .has_command("nixpacks")
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
            .shell_installer("https://nixpacks.com/install.sh")
            .shell("bash")
            .env("NIXPACKS_VERSION", "1.41.0")
            .run()
            .await?;
        self.executor.run("nixpacks", ["--version"]).await
    }

    pub async fn if_not_exist_install(&self) -> ExecResult<()> {
        if !self.exists().await {
            self.install().await?;
        }
        Ok(())
    }

    pub fn build(&self, path: impl Into<String>) -> NixpacksBuildBuilder<'_> {
        let mut args = ArgBuilder::cmd(&["build"]);
        args.push(path.into());
        NixpacksBuildBuilder {
            executor: self.executor,
            args,
        }
    }
}

pub struct NixpacksBuildBuilder<'a> {
    executor: &'a CommandExecutor,
    args: ArgBuilder,
}

impl<'a> NixpacksBuildBuilder<'a> {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.args.pair("--name", name.into());
        self
    }

    pub fn install_cmd(mut self, cmd: impl Into<String>) -> Self {
        self.args.pair("--install-cmd", cmd.into());
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

    pub fn env(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self {
        self.args
            .pair("--env", format!("{}={}", k.as_ref(), v.as_ref()));
        self
    }

    pub fn pkgs(mut self, pkgs: impl IntoIterator<Item = impl Into<String>>) -> Self {
        for pkg in pkgs {
            self.args.pair("--pkgs", pkg.into());
        }
        self
    }

    pub fn apt(mut self, pkgs: impl IntoIterator<Item = impl Into<String>>) -> Self {
        for pkg in pkgs {
            self.args.pair("--apt", pkg.into());
        }
        self
    }

    pub fn libs(mut self, libs: impl IntoIterator<Item = impl Into<String>>) -> Self {
        for lib in libs {
            self.args.pair("--libs", lib.into());
        }
        self
    }

    pub fn config(mut self, file: impl Into<String>) -> Self {
        self.args.pair("--config", file.into());
        self
    }

    pub fn no_cache(mut self) -> Self {
        self.args.flag("--no-cache");
        self
    }

    pub fn out(mut self, dir: impl Into<String>) -> Self {
        self.args.pair("--out", dir.into());
        self
    }

    pub fn tag(mut self, tags: impl IntoIterator<Item = impl Into<String>>) -> Self {
        for tag in tags {
            self.args.pair("--tag", tag.into());
        }
        self
    }

    pub fn label(mut self, labels: impl IntoIterator<Item = impl Into<String>>) -> Self {
        for label in labels {
            self.args.pair("--label", label.into());
        }
        self
    }

    pub fn cache_key(mut self, key: impl Into<String>) -> Self {
        self.args.pair("--cache-key", key.into());
        self
    }

    pub fn platform(mut self, platforms: impl IntoIterator<Item = impl Into<String>>) -> Self {
        for platform in platforms {
            self.args.pair("--platform", platform.into());
        }
        self
    }

    pub fn cache_from(mut self, cache_from: impl Into<String>) -> Self {
        self.args.pair("--cache-from", cache_from.into());
        self
    }

    pub fn inline_cache(mut self) -> Self {
        self.args.flag("--inline-cache");
        self
    }

    pub fn docker_host(mut self, host: impl Into<String>) -> Self {
        self.args.pair("--docker-host", host.into());
        self
    }

    pub fn docker_tls_verify(mut self) -> Self {
        self.args.flag("--docker-tls-verify");
        self
    }

    pub fn docker_cert_path(mut self, path: impl Into<String>) -> Self {
        self.args.pair("--docker-cert-path", path.into());
        self
    }

    pub async fn run(self, cancel: &CancellationToken) -> ExecResult<ExecOutput> {
        self.executor
            .run_cancelled("nixpacks", self.args.build(), cancel)
            .await
    }

    pub async fn run_in_cgroup(
        self,
        cgroup_path: Option<&str>,
        cancel: &CancellationToken,
    ) -> ExecResult<ExecOutput> {
        self.executor
            .run_cancelled_in_cgroup(cgroup_path, "nixpacks", self.args.build(), cancel)
            .await
    }
}

impl crate::utils::exec::pipeline::IntoCommand for NixpacksBuildBuilder<'_> {
    fn build_str(&self) -> String {
        format!("nixpacks {}", self.args.build_string())
    }
}
