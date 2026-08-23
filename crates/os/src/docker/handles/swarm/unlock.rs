use crate::docker::{DockerOutput, DockerResult, client::DockerCli, core::ArgBuilder};

pub struct SwarmUnlockKeyBuilder<'a> {
    cli: &'a DockerCli,
    api: bool,
}

impl<'a> SwarmUnlockKeyBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self { cli, api: false }
    }
    pub fn api(mut self) -> Self {
        self.api = true;
        self
    }

    pub async fn get(&self) -> DockerResult<String> {
        if self.api {
            return Ok(
                self.cli.api_get_json("/swarm/unlockkey").await?["UnlockKey"]
                    .as_str()
                    .unwrap_or_default()
                    .to_owned(),
            );
        }
        let out = self.cli.run(["swarm", "unlock-key", "--quiet"]).await?;
        Ok(out.stdout.trim().to_string())
    }

    pub async fn rotate(&self) -> DockerResult<String> {
        if self.api {
            let swarm = self.cli.api_get_json("/swarm").await?;
            let version = swarm["Version"]["Index"].as_u64().unwrap_or(0);
            self.cli
                .api_json_request(
                    &format!("/swarm/update?version={version}&rotateManagerUnlockKey=true"),
                    swarm["Spec"].clone(),
                )
                .await?;
            return Ok(
                self.cli.api_get_json("/swarm/unlockkey").await?["UnlockKey"]
                    .as_str()
                    .unwrap_or_default()
                    .to_owned(),
            );
        }
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
        if self.args.api {
            return self
                .cli
                .api_json_request("/swarm/unlock", serde_json::json!({"UnlockKey": self.key}))
                .await;
        }
        let a = self.args.build();
        let refs: Vec<&str> = a.iter().map(String::as_str).collect();
        // Securely pass the key via stdin
        self.cli.run_with_stdin(refs, self.key.as_bytes()).await
    }
}
crate::impl_builder_opts!(SwarmUnlockBuilder);
