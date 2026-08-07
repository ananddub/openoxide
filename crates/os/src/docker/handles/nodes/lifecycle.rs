use crate::docker::{DockerOutput, DockerResult, client::DockerCli, core::ArgBuilder};

// Promote
pub struct NodePromoteBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    node_id: String,
}

impl<'a> NodePromoteBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, node_id: impl Into<String>) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["node", "promote"]),
            node_id: node_id.into(),
        }
    }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        self.args.push(&self.node_id);
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(NodePromoteBuilder);

// Demote
pub struct NodeDemoteBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    node_id: String,
}

impl<'a> NodeDemoteBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, node_id: impl Into<String>) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["node", "demote"]),
            node_id: node_id.into(),
        }
    }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        self.args.push(&self.node_id);
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(NodeDemoteBuilder);

// Remove
pub struct NodeRemoveBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    node_id: String,
}

impl<'a> NodeRemoveBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, node_id: impl Into<String>) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["node", "rm"]),
            node_id: node_id.into(),
        }
    }

    pub fn force(mut self) -> Self {
        self.args.flag("--force");
        self
    }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        self.args.push(&self.node_id);
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(NodeRemoveBuilder);
