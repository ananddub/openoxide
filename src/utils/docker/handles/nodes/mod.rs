use crate::utils::docker::{DockerResult, client::DockerCli};

pub use lifecycle::{NodeDemoteBuilder, NodePromoteBuilder, NodeRemoveBuilder};
pub use list::NodeListBuilder;
pub use ps::NodePsBuilder;
pub use update::NodeUpdateBuilder;

// ── NodesHandle ─────────────────────────────────────────────────────────────

pub struct NodesHandle<'a> {
    cli: &'a DockerCli,
}

impl<'a> NodesHandle<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self { cli }
    }

    pub fn update(&self, node_id: impl Into<String>) -> NodeUpdateBuilder<'a> {
        NodeUpdateBuilder::new(self.cli, node_id)
    }

    pub fn promote(&self, node_id: impl Into<String>) -> NodePromoteBuilder<'a> {
        NodePromoteBuilder::new(self.cli, node_id)
    }

    pub fn demote(&self, node_id: impl Into<String>) -> NodeDemoteBuilder<'a> {
        NodeDemoteBuilder::new(self.cli, node_id)
    }

    pub fn remove(&self, node_id: impl Into<String>) -> NodeRemoveBuilder<'a> {
        NodeRemoveBuilder::new(self.cli, node_id)
    }

    pub fn list(&self) -> NodeListBuilder<'a> {
        NodeListBuilder::new(self.cli)
    }

    pub fn ps(&self, node_id: impl Into<String>) -> NodePsBuilder<'a> {
        NodePsBuilder::new(self.cli, node_id)
    }

    pub async fn inspect(&self, node_id: impl AsRef<str>) -> DockerResult<crate::utils::docker::NodeInspect> {
        let out = self.cli.run(["node", "inspect", node_id.as_ref()]).await?;
        let mut json: Vec<crate::utils::docker::NodeInspect> = serde_json::from_str(&out.stdout)?;
        Ok(json.pop().unwrap_or_default())
    }

    pub async fn inspect_raw(&self, node_id: impl AsRef<str>) -> DockerResult<serde_json::Value> {
        let out = self.cli.run(["node", "inspect", node_id.as_ref()]).await?;
        let mut json: Vec<serde_json::Value> = serde_json::from_str(&out.stdout)?;
        Ok(json.pop().unwrap_or_default())
    }
}

pub mod lifecycle;
pub mod list;
pub mod ps;
pub mod update;
