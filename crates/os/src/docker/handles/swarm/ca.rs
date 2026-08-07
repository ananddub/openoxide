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
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(SwarmCaBuilder);
