use crate::utils::docker::{DockerOutput, DockerResult, client::DockerCli, core::ArgBuilder};
use tokio_util::sync::CancellationToken;

pub use ca::SwarmCaBuilder;
pub use init::SwarmInitBuilder;
pub use join::{SwarmJoinBuilder, SwarmJoinTokenBuilder};
pub use leave::SwarmLeaveBuilder;
pub use unlock::{SwarmUnlockBuilder, SwarmUnlockKeyBuilder};
pub use update::SwarmUpdateBuilder;

// ── SwarmHandle ─────────────────────────────────────────────────────────────

pub struct SwarmHandle<'a> {
    cli: &'a DockerCli,
}

impl<'a> SwarmHandle<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self { cli }
    }

    pub fn init(&self) -> SwarmInitBuilder<'a> {
        SwarmInitBuilder::new(self.cli)
    }

    pub fn join(&self) -> SwarmJoinBuilder<'a> {
        SwarmJoinBuilder::new(self.cli)
    }

    pub fn leave(&self) -> SwarmLeaveBuilder<'a> {
        SwarmLeaveBuilder::new(self.cli)
    }

    pub fn update(&self) -> SwarmUpdateBuilder<'a> {
        SwarmUpdateBuilder::new(self.cli)
    }

    pub fn unlock_key(&self) -> SwarmUnlockKeyBuilder<'a> {
        SwarmUnlockKeyBuilder::new(self.cli)
    }

    pub fn join_token(&self) -> SwarmJoinTokenBuilder<'a> {
        SwarmJoinTokenBuilder::new(self.cli)
    }

    pub fn unlock(&self, key: impl Into<String>) -> SwarmUnlockBuilder<'a> {
        SwarmUnlockBuilder::new(self.cli, key)
    }

    pub fn ca(&self) -> SwarmCaBuilder<'a> {
        SwarmCaBuilder::new(self.cli)
    }

    pub fn active(&self) -> SwarmActiveBuilder<'a> {
        SwarmActiveBuilder::new(self.cli)
    }

    pub async fn inspect(&self) -> DockerResult<crate::utils::docker::SwarmInfo> {
        self.cli
            .json(&["info", "--format", "{{json .Swarm}}"])
            .await
    }

    /// Same as [`inspect`](Self::inspect), but abortable — use this inside
    /// build/deploy pipelines that honor a [`CancellationToken`].
    pub async fn inspect_cancelled(
        &self,
        cancel: &CancellationToken,
    ) -> DockerResult<crate::utils::docker::SwarmInfo> {
        self.cli
            .json_cancelled(&["info", "--format", "{{json .Swarm}}"], cancel)
            .await
    }
}

pub struct SwarmActiveBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
}

impl<'a> SwarmActiveBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        let mut args = ArgBuilder::cmd(&["info"]);
        args.pair("--format", "{{.Swarm.LocalNodeState}}");
        Self { cli, args }
    }

    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }
}

impl crate::utils::exec::script::IntoCommand for SwarmActiveBuilder<'_> {
    fn build_str(&self) -> String {
        format!("{} | grep -q '^active$'", self.args.preview())
    }
}

pub mod ca;
pub mod init;
pub mod join;
pub mod leave;
pub mod unlock;
pub mod update;
