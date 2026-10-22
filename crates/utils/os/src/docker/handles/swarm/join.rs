use crate::docker::{
    DockerOutput, DockerResult,
    client::DockerCli,
    core::{ArgBuilder, types::SwarmRole},
};

pub struct SwarmJoinBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    remote: Option<String>,
}

impl<'a> SwarmJoinBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["swarm", "join"]),
            remote: None,
        }
    }

    pub fn remote(mut self, remote: impl Into<String>) -> Self {
        self.remote = Some(remote.into());
        self
    }
    pub fn token(mut self, token: impl AsRef<str>) -> Self {
        self.args.pair("--token", token);
        self
    }
    pub fn advertise_addr(mut self, addr: impl AsRef<str>) -> Self {
        self.args.pair("--advertise-addr", addr);
        self
    }
    pub fn listen_addr(mut self, addr: impl AsRef<str>) -> Self {
        self.args.pair("--listen-addr", addr);
        self
    }
    pub fn data_path_port(mut self, port: u16) -> Self {
        self.args.pair("--data-path-port", port.to_string());
        self
    }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        if let Some(r) = self.remote.take() {
            self.args.push(r);
        }
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(SwarmJoinBuilder);

pub struct SwarmJoinTokenBuilder<'a> {
    cli: &'a DockerCli,
    api: bool,
}

impl<'a> SwarmJoinTokenBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self { cli, api: false }
    }
    pub fn api(mut self) -> Self {
        self.api = true;
        self
    }

    pub async fn get(&self, role: SwarmRole) -> DockerResult<String> {
        if self.api {
            return swarm_token(self.cli, role, false).await;
        }
        let out = self
            .cli
            .run(["swarm", "join-token", "--quiet", &role.to_string()])
            .await?;
        Ok(out.stdout.trim().to_string())
    }

    pub async fn rotate(&self, role: SwarmRole) -> DockerResult<String> {
        if self.api {
            return swarm_token(self.cli, role, true).await;
        }
        let out = self
            .cli
            .run([
                "swarm",
                "join-token",
                "--rotate",
                "--quiet",
                &role.to_string(),
            ])
            .await?;
        Ok(out.stdout.trim().to_string())
    }
}

async fn swarm_token(cli: &DockerCli, role: SwarmRole, rotate: bool) -> DockerResult<String> {
    let swarm = cli.api_get_json("/swarm").await?;
    if rotate {
        let version = swarm["Version"]["Index"].as_u64().unwrap_or(0);
        let spec = swarm["Spec"].clone();
        cli.api_json_request(
            &format!(
                "/swarm/update?version={version}&rotateWorkerToken={}&rotateManagerToken={}",
                role.to_string() == "worker",
                role.to_string() == "manager"
            ),
            spec,
        )
        .await?;
    }
    let swarm = cli.api_get_json("/swarm").await?;
    let key = if role.to_string() == "manager" {
        "Manager"
    } else {
        "Worker"
    };
    Ok(swarm["JoinTokens"][key]
        .as_str()
        .unwrap_or_default()
        .to_owned())
}
