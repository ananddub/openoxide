use crate::docker::{DockerOutput, DockerResult, client::DockerCli, core::ArgBuilder};

pub struct SwarmInitBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
}

impl<'a> SwarmInitBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["swarm", "init"]),
        }
    }

    pub fn advertise_addr(mut self, addr: impl AsRef<str>) -> Self {
        self.args.pair("--advertise-addr", addr);
        self
    }
    pub fn listen_addr(mut self, addr: impl AsRef<str>) -> Self {
        self.args.pair("--listen-addr", addr);
        self
    }
    pub fn force_new_cluster(mut self) -> Self {
        self.args.flag("--force-new-cluster");
        self
    }
    pub fn data_path_port(mut self, port: u16) -> Self {
        self.args.pair("--data-path-port", port.to_string());
        self
    }
    pub fn default_addr_pool(mut self, pool: impl AsRef<str>) -> Self {
        self.args.pair("--default-addr-pool", pool);
        self
    }
    pub fn default_addr_pool_mask_length(mut self, len: u8) -> Self {
        self.args
            .pair("--default-addr-pool-mask-length", len.to_string());
        self
    }

    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(SwarmInitBuilder);

impl crate::exec::script::IntoCommand for SwarmInitBuilder<'_> {
    fn build_str(&self) -> String {
        self.args.preview()
    }
}
