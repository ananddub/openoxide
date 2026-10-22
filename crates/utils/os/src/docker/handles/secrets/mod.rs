use crate::docker::{DockerResult, client::DockerCli};

pub use create::SecretCreateBuilder;
pub use list::SecretListBuilder;
pub use remove::SecretRemoveBuilder;

// ── SecretsHandle ───────────────────────────────────────────────────────────

pub struct SecretsHandle<'a> {
    cli: &'a DockerCli,
}

impl<'a> SecretsHandle<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self { cli }
    }

    pub fn create(&self, name: impl Into<String>) -> SecretCreateBuilder<'a> {
        SecretCreateBuilder::new(self.cli, name)
    }

    pub fn remove(&self, name: impl Into<String>) -> SecretRemoveBuilder<'a> {
        SecretRemoveBuilder::new(self.cli, name)
    }

    pub fn list(&self) -> SecretListBuilder<'a> {
        SecretListBuilder::new(self.cli)
    }

    pub async fn inspect(
        &self,
        name: impl AsRef<str>,
    ) -> DockerResult<crate::docker::SecretInspect> {
        let out = self.cli.run(["secret", "inspect", name.as_ref()]).await?;
        let mut json: Vec<crate::docker::SecretInspect> = serde_json::from_str(&out.stdout)?;
        Ok(json.pop().unwrap_or_default())
    }

    pub async fn inspect_raw(&self, name: impl AsRef<str>) -> DockerResult<serde_json::Value> {
        let out = self.cli.run(["secret", "inspect", name.as_ref()]).await?;
        let mut json: Vec<serde_json::Value> = serde_json::from_str(&out.stdout)?;
        Ok(json.pop().unwrap_or_default())
    }
}

pub mod create;
pub mod list;
pub mod remove;
