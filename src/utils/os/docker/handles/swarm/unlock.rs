use crate::utils::docker::{DockerOutput, DockerResult, client::DockerCli, core::ArgBuilder};

pub struct SwarmUnlockKeyBuilder<'a> {
    cli: &'a DockerCli,
}

impl<'a> SwarmUnlockKeyBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self { cli }
    }

    pub async fn get(&self) -> DockerResult<String> {
        let out = self.cli.run(["swarm", "unlock-key", "--quiet"]).await?;
        Ok(out.stdout.trim().to_string())
    }

    pub async fn rotate(&self) -> DockerResult<String> {
        let out = self
            .cli
            .run(["swarm", "unlock-key", "--rotate", "--quiet"])
            .await?;
        Ok(out.stdout.trim().to_string())
    }
}

pub struct SwarmUnlockBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    key: String,
}

impl<'a> SwarmUnlockBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, key: impl Into<String>) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["swarm", "unlock"]),
            key: key.into(),
        }
    }

    pub async fn run(self) -> DockerResult<DockerOutput> {
        let a = self.args.build();
        let refs: Vec<&str> = a.iter().map(String::as_str).collect();
        // Securely pass the key via stdin
        self.cli.run_with_stdin(refs, self.key.as_bytes()).await
    }
}
