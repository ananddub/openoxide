use crate::docker::{
    DockerOutput, DockerResult,
    client::DockerCli,
    core::{
        ArgBuilder,
        types::{NodeAvailability, NodeRole},
    },
};

pub struct NodeUpdateBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    node_id: String,
}

impl<'a> NodeUpdateBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, node_id: impl Into<String>) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["node", "update"]),
            node_id: node_id.into(),
        }
    }

    pub fn availability(mut self, avail: NodeAvailability) -> Self {
        self.args.pair("--availability", avail.to_string());
        self
    }
    pub fn role(mut self, role: NodeRole) -> Self {
        self.args.pair("--role", role.to_string());
        self
    }
    pub fn label(mut self, key: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        self.args.pair(
            "--label-add",
            format!("{}={}", key.as_ref(), value.as_ref()),
        );
        self
    }
    pub fn remove_label(mut self, key: impl AsRef<str>) -> Self {
        self.args.pair("--label-rm", key);
        self
    }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        self.args.push(&self.node_id);
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(NodeUpdateBuilder);
