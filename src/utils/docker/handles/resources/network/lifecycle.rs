use crate::utils::docker::{
    DockerCli, DockerOutput, DockerResult, core::ArgBuilder, query::filter::NetworkFilter,
};

pub struct NetworkPrune<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) args: ArgBuilder,
}

impl<'a> NetworkPrune<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["network", "prune", "--force"]),
        }
    }
    pub fn filter(mut self, f: NetworkFilter) -> Self {
        self.args.filter(f);
        self
    }
    pub fn print(&self) -> String {
        self.args.preview()
    }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(NetworkPrune);

pub struct NetworkRmBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) name: String,
    pub(crate) args: ArgBuilder,
}

impl<'a> NetworkRmBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, name: impl Into<String>) -> Self {
        Self {
            cli,
            name: name.into(),
            args: ArgBuilder::cmd(&["network", "rm"]),
        }
    }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let mut a = self.args;
        a.push(&self.name);
        self.cli.execute(&a).await
    }
}
crate::impl_builder_opts!(NetworkRmBuilder);

pub struct NetworkConnectBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) network: String,
    pub(crate) container: String,
    pub(crate) args: ArgBuilder,
}

impl<'a> NetworkConnectBuilder<'a> {
    pub(crate) fn new(
        cli: &'a DockerCli,
        network: impl Into<String>,
        container: impl Into<String>,
    ) -> Self {
        Self {
            cli,
            network: network.into(),
            container: container.into(),
            args: ArgBuilder::cmd(&["network", "connect"]),
        }
    }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let mut a = self.args;
        a.push(&self.network);
        a.push(&self.container);
        self.cli.execute(&a).await
    }
}
crate::impl_builder_opts!(NetworkConnectBuilder);

pub struct NetworkDisconnectBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) network: String,
    pub(crate) container: String,
    pub(crate) args: ArgBuilder,
}

impl<'a> NetworkDisconnectBuilder<'a> {
    pub(crate) fn new(
        cli: &'a DockerCli,
        network: impl Into<String>,
        container: impl Into<String>,
    ) -> Self {
        Self {
            cli,
            network: network.into(),
            container: container.into(),
            args: ArgBuilder::cmd(&["network", "disconnect"]),
        }
    }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        let mut a = self.args;
        a.push(&self.network);
        a.push(&self.container);
        self.cli.execute(&a).await
    }
}
crate::impl_builder_opts!(NetworkDisconnectBuilder);
