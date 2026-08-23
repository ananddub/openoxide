use crate::docker::{DockerOutput, DockerResult, client::DockerCli, core::ArgBuilder};

pub struct SwarmCaBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
}

impl<'a> SwarmCaBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["swarm", "ca"]),
        }
    }

    pub fn rotate(mut self) -> Self {
        self.args.flag("--rotate");
        self
    }
    pub fn external_ca(mut self, ca: impl AsRef<str>) -> Self {
        self.args.pair("--external-ca", ca);
        self
    }
    pub fn ca_cert(mut self, cert: impl AsRef<str>) -> Self {
        self.args.pair("--ca-cert", cert);
        self
    }
    pub fn ca_key(mut self, key: impl AsRef<str>) -> Self {
        self.args.pair("--ca-key", key);
        self
    }

    pub async fn run(self) -> DockerResult<DockerOutput> {
        if self.args.api {
            let swarm = self.cli.api_get_json("/swarm").await?;
            let version = swarm["Version"]["Index"].as_u64().unwrap_or(0);
            let mut spec = swarm["Spec"].clone();
            let built = self.args.clone().build();
            if built.iter().any(|arg| arg == "--rotate") {
                let force = spec["CAConfig"]["ForceRotate"].as_u64().unwrap_or(0);
                spec["CAConfig"]["ForceRotate"] = serde_json::json!(force + 1);
            }
            for pair in built.windows(2) {
                match pair[0].as_str() {
                    "--external-ca" => {
                        if !spec["CAConfig"]["ExternalCAs"].is_array() {
                            spec["CAConfig"]["ExternalCAs"] = serde_json::json!([]);
                        }
                        spec["CAConfig"]["ExternalCAs"]
                            .as_array_mut()
                            .unwrap()
                            .push(serde_json::json!({"Protocol": "cfssl", "URL": pair[1]}));
                    }
                    "--ca-cert" => {
                        spec["CAConfig"]["SigningCACert"] =
                            serde_json::json!(tokio::fs::read_to_string(&pair[1]).await?)
                    }
                    "--ca-key" => {
                        spec["CAConfig"]["SigningCAKey"] =
                            serde_json::json!(tokio::fs::read_to_string(&pair[1]).await?)
                    }
                    _ => {}
                }
            }
            return self
                .cli
                .api_json_request(&format!("/swarm/update?version={version}"), spec)
                .await;
        }
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(SwarmCaBuilder);
