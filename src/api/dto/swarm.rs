use serde::{Deserialize, Serialize};

use crate::utils::docker::types::NodeSummary;

// ------------------------------------------------------------------ //
//  Request DTOs                                                        //
// ------------------------------------------------------------------ //

/// Connection details needed to talk to a remote server's Docker daemon.
#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct SwarmConnectionDto {
    /// Server DB id — used to resolve SSH credentials.
    pub server_id: Option<i64>,
    /// Force the operation when Docker requires explicit confirmation.
    ///
    /// Used by swarm leave for manager nodes. Keep this false for normal
    /// leave actions; set true only when intentionally resetting a node so it
    /// can join another cluster.
    pub force: Option<bool>,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct NodeActionDto {
    /// Server id of the node to act on.
    pub server_id: Option<i64>,
    /// Node id (from docker node ls).
    pub node_id: String,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct NodeAvailabilityDto {
    pub server_id: Option<i64>,
    pub node_id: String,
    /// active | pause | drain
    pub availability: String,
}

// ------------------------------------------------------------------ //
//  Response DTOs                                                       //
// ------------------------------------------------------------------ //

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct SwarmTokensDto {
    pub worker: String,
    pub manager: String,
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct SwarmInfoDto {
    pub node_id: String,
    pub node_addr: String,
    pub local_node_state: String,
    pub control_available: bool,
    pub nodes: i64,
    pub managers: i64,
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct NodeDto {
    pub id: String,
    pub hostname: String,
    pub status: String,
    pub availability: String,
    pub manager_status: String,
    pub engine_version: String,
}

impl From<NodeSummary> for NodeDto {
    fn from(n: NodeSummary) -> Self {
        Self {
            id: n.id,
            hostname: n.hostname,
            status: n.status,
            availability: n.availability,
            manager_status: n.manager_status,
            engine_version: n.engine_version,
        }
    }
}
