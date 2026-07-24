use crate::utils::docker::{
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
    pub fn arg(mut self, v: impl Into<String>) -> Self {
        self.args.push(v.into());
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
}

impl<'a> SwarmJoinTokenBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self { cli }
    }

    pub async fn get(&self, role: SwarmRole) -> DockerResult<String> {
        let out = self
            .cli
            .run(["swarm", "join-token", "--quiet", &role.to_string()])
            .await?;
        Ok(out.stdout.trim().to_string())
    }

    pub async fn rotate(&self, role: SwarmRole) -> DockerResult<String> {
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
