use crate::string_enum;
use crate::utils::exec::{ArgBuilder, CommandExecutor, ExecOutput, ExecResult};
use crate::utils::os::OsCli;
use tokio_util::sync::CancellationToken;

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum PaketoBuilderImage {
        default = JammyFull;

        JammyFull => "paketobuildpacks/builder-jammy-full",
        JammyBase => "paketobuildpacks/builder-jammy-base",
        JammyTiny => "paketobuildpacks/builder-jammy-tiny",
        BionicFull => "paketobuildpacks/builder-bionic-full",
        BionicBase => "paketobuildpacks/builder-bionic-base",
    }
}

#[derive(Clone, Debug)]
pub struct PackCli<'a> {
    pub(crate) executor: &'a CommandExecutor,
}

impl<'a> PackCli<'a> {
    pub fn new(executor: &'a CommandExecutor) -> Self {
        Self { executor }
    }

    pub async fn exists(&self) -> bool {
        OsCli::new(self.executor)
            .has_command("pack")
            .run()
            .await
            .map(|out| out.success())
            .unwrap_or(false)
    }

    pub async fn is_exists(&self) -> bool {
        self.exists().await
    }

    pub async fn install(&self) -> ExecResult<ExecOutput> {
        let os = OsCli::new(self.executor);
        os.pack_installer("0.39.1").run().await?;
        self.executor.run("pack", ["--version"]).await
    }

    pub async fn if_not_exist_install(&self) -> ExecResult<()> {
        if !self.exists().await {
            self.install().await?;
        }
        Ok(())
    }

    pub fn build(&self, image_name: impl Into<String>) -> PackBuildBuilder<'_> {
        let mut args = ArgBuilder::cmd(&["build"]);
        args.push(image_name.into());
        PackBuildBuilder {
            executor: self.executor,
            args,
        }
    }
}

pub struct PackBuildBuilder<'a> {
    executor: &'a CommandExecutor,
    args: ArgBuilder,
}

impl<'a> PackBuildBuilder<'a> {
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.args.pair("--path", path.into());
        self
    }

    pub fn builder(mut self, builder_image: PaketoBuilderImage) -> Self {
        self.args.pair("--builder", builder_image.as_str());
        self
    }

    pub fn buildpack(mut self, buildpacks: impl IntoIterator<Item = impl Into<String>>) -> Self {
        for bp in buildpacks {
            self.args.pair("--buildpack", bp.into());
        }
        self
    }

    pub fn cache_image(mut self, image: impl Into<String>) -> Self {
        self.args.pair("--cache-image", image.into());
        self
    }

    pub fn clear_cache(mut self) -> Self {
        self.args.flag("--clear-cache");
        self
    }

    pub fn publish(mut self) -> Self {
        self.args.flag("--publish");
        self
    }

    pub fn buildpack_registry(mut self, registry: impl Into<String>) -> Self {
        self.args.pair("--buildpack-registry", registry.into());
        self
    }

    pub fn env(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self {
        self.args
            .pair("--env", format!("{}={}", k.as_ref(), v.as_ref()));
        self
    }

    pub fn network(mut self, network: impl Into<String>) -> Self {
        self.args.pair("--network", network.into());
        self
    }

    pub fn volume(mut self, volume: impl Into<String>) -> Self {
        self.args.pair("--volume", volume.into());
        self
    }

    pub fn cache(mut self, cache_config: impl Into<String>) -> Self {
        self.args.pair("--cache", cache_config.into());
        self
    }

    pub fn trust_extra_buildpacks(mut self) -> Self {
        self.args.flag("--trust-extra-buildpacks");
        self
    }

    pub fn workspace(mut self, path: impl Into<String>) -> Self {
        self.args.pair("--workspace", path.into());
        self
    }

    pub fn uid(mut self, uid: u32) -> Self {
        self.args.pair("--uid", uid.to_string());
        self
    }

    pub fn gid(mut self, gid: u32) -> Self {
        self.args.pair("--gid", gid.to_string());
        self
    }

    pub async fn run(self, cancel: &CancellationToken) -> ExecResult<ExecOutput> {
        self.executor
            .run_cancelled("pack", self.args.build(), cancel)
            .await
    }

    pub async fn run_in_cgroup(
        self,
        self_cgroup_path: Option<&str>,
        cancel: &CancellationToken,
    ) -> ExecResult<ExecOutput> {
        self.executor
            .run_cancelled_in_cgroup(self_cgroup_path, "pack", self.args.build(), cancel)
            .await
    }
}

impl crate::utils::exec::pipeline::IntoCommand for PackBuildBuilder<'_> {
    fn build_str(&self) -> String {
        format!("pack {}", self.args.build_string())
    }
}
