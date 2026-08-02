use serde::{Deserialize, Serialize};

use crate::utils::docker::NodeInspect;
use crate::utils::docker::core::types::{NodeRole, SwarmRole};

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

/// Joins one server into another server's existing Swarm cluster, in one
/// shot: force-leaves any standalone swarm the target is already part of,
/// then joins it to the manager's cluster with a VPN-aware advertise
/// address.
#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct SwarmJoinDto {
    /// Server id of the node that should join the cluster.
    pub target_server_id: i64,
    /// Server id whose cluster to join. None = local engine's cluster.
    pub manager_server_id: Option<i64>,
    pub role: SwarmRole,
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
    /// From `Spec.Role`, the authoritative config value. Doesn't flip based
    /// on reachability, unlike `docker node ls`'s ManagerStatus column.
    pub role: NodeRole,
    pub is_leader: bool,
    /// "reachable" | "unreachable" | "" (empty for workers).
    pub reachability: String,
    pub ip_address: String,
    pub engine_version: String,
}

impl From<NodeInspect> for NodeDto {
    fn from(n: NodeInspect) -> Self {
        let (is_leader, reachability) = n
            .manager_status
            .map(|m| (m.leader, m.reachability))
            .unwrap_or_default();
        Self {
            id: n.id,
            hostname: n.description.hostname,
            status: n.status.state,
            availability: n.spec.availability,
            role: n.spec.role,
            is_leader,
            reachability,
            ip_address: n.status.addr,
            engine_version: n.description.engine.engine_version,
        }
    }
}
