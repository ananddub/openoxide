use crate::utils::docker::{DockerResult, client::DockerCli};

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

    pub async fn inspect(&self) -> DockerResult<crate::utils::docker::SwarmInfo> {
        let output = self
            .cli
            .run(["info", "--format", "{{json .Swarm}}"])
            .await?;
        let json = serde_json::from_str(&output.stdout)?;
        Ok(json)
    }
}

pub mod ca;
pub mod init;
pub mod join;
pub mod leave;
pub mod unlock;
pub mod update;
