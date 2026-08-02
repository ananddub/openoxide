use crate::utils::docker::core::types::NodeRole;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Default + serde::Deserialize<'de>,
{
    let opt = Option::<T>::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectVersion {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub index: u64,
}

// ==========================================
// 1. Container Inspect Schema
// ==========================================

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInspect {
    #[serde(rename = "Id", default, deserialize_with = "deserialize_null_default")]
    pub id: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub created: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub path: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub args: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub state: ContainerInspectState,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub image: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub name: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub restart_count: i64,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub driver: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub platform: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub mount_label: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub process_label: String,
    #[serde(
        rename = "ExecIDs",
        default,
        deserialize_with = "deserialize_null_default"
    )]
    pub exec_ids: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub host_config: serde_json::Value,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub config: ContainerInspectConfig,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub network_settings: ContainerInspectNetworkSettings,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub mounts: Vec<ContainerInspectMount>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInspectState {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub status: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub running: bool,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub paused: bool,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub restarting: bool,
    #[serde(
        rename = "OOMKilled",
        default,
        deserialize_with = "deserialize_null_default"
    )]
    pub oom_killed: bool,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub dead: bool,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub pid: i64,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub exit_code: i32,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub error: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub started_at: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub finished_at: String,
    #[serde(default)]
    pub health: Option<ContainerHealthInspect>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerHealthInspect {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub status: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub failing_streak: i64,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInspectConfig {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub hostname: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub domainname: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub user: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub env: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub cmd: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub image: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub working_dir: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub entrypoint: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub labels: HashMap<String, String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInspectNetworkSettings {
    #[serde(
        rename = "IPAddress",
        default,
        deserialize_with = "deserialize_null_default"
    )]
    pub ip_address: String,
    #[serde(
        rename = "IPPrefixLen",
        default,
        deserialize_with = "deserialize_null_default"
    )]
    pub ip_prefix_len: u32,
    #[serde(
        rename = "Gateway",
        default,
        deserialize_with = "deserialize_null_default"
    )]
    pub gateway: String,
    #[serde(
        rename = "MacAddress",
        default,
        deserialize_with = "deserialize_null_default"
    )]
    pub mac_address: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub ports: HashMap<String, Option<Vec<ContainerInspectPortBinding>>>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub networks: HashMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInspectPortBinding {
    #[serde(
        rename = "HostIp",
        default,
        deserialize_with = "deserialize_null_default"
    )]
    pub host_ip: String,
    #[serde(
        rename = "HostPort",
        default,
        deserialize_with = "deserialize_null_default"
    )]
    pub host_port: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInspectMount {
    #[serde(
        rename = "Type",
        default,
        deserialize_with = "deserialize_null_default"
    )]
    pub mount_type: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub name: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub source: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub destination: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub driver: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub mode: String,
    #[serde(rename = "RW", default, deserialize_with = "deserialize_null_default")]
    pub rw: bool,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub propagation: String,
}

// ==========================================
// 2. Swarm Service Inspect Schema
// ==========================================

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceInspect {
    #[serde(rename = "ID", default, deserialize_with = "deserialize_null_default")]
    pub id: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub version: ObjectVersion,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub created_at: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub updated_at: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub spec: ServiceSpec,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub endpoint: ServiceEndpoint,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceSpec {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub name: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub labels: HashMap<String, String>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub mode: HashMap<String, serde_json::Value>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub task_template: serde_json::Value,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceEndpoint {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub spec: serde_json::Value,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub ports: Vec<serde_json::Value>,
}

// ==========================================
// 3. Network Inspect Schema
// ==========================================

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkInspect {
    #[serde(rename = "Id", default, deserialize_with = "deserialize_null_default")]
    pub id: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub name: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub created: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub scope: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub driver: String,
    #[serde(
        rename = "EnableIPv6",
        default,
        deserialize_with = "deserialize_null_default"
    )]
    pub enable_ipv6: bool,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub internal: bool,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub attachable: bool,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub ingress: bool,
    #[serde(
        rename = "IPAM",
        default,
        deserialize_with = "deserialize_null_default"
    )]
    pub ipam: serde_json::Value,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub containers: HashMap<String, serde_json::Value>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub options: HashMap<String, String>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub labels: HashMap<String, String>,
}

// ==========================================
// 4. Volume Inspect Schema
// ==========================================

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct VolumeInspect {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub name: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub driver: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub mountpoint: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub created_at: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub status: HashMap<String, serde_json::Value>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub labels: HashMap<String, String>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub scope: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub options: HashMap<String, String>,
}

// ==========================================
// 5. Image Inspect Schema
// ==========================================

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ImageInspect {
    #[serde(rename = "Id", default, deserialize_with = "deserialize_null_default")]
    pub id: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub repo_tags: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub repo_digests: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub created: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub size: u64,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub virtual_size: u64,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub architecture: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub os: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub config: ContainerInspectConfig,
}

// ==========================================
// 6. Node Inspect Schema
// ==========================================

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct NodeInspect {
    #[serde(rename = "ID", default, deserialize_with = "deserialize_null_default")]
    pub id: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub version: ObjectVersion,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub created_at: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub updated_at: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub spec: NodeSpec,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub description: NodeDescription,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub status: NodeStatus,
    /// Only present for manager nodes — absent entirely for workers.
    #[serde(default)]
    pub manager_status: Option<NodeManagerStatus>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct NodeDescription {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub hostname: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub engine: NodeEngine,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct NodeEngine {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub engine_version: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct NodeManagerStatus {
    #[serde(default)]
    pub leader: bool,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub reachability: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub addr: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct NodeSpec {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub name: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub labels: HashMap<String, String>,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub role: NodeRole,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub availability: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct NodeStatus {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub state: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub addr: String,
}

// ==========================================
// 7. Secret & Config Inspect Schemas
// ==========================================

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct SecretInspect {
    #[serde(rename = "ID", default, deserialize_with = "deserialize_null_default")]
    pub id: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub version: ObjectVersion,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub created_at: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub updated_at: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub spec: SecretSpec,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct SecretSpec {
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub name: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub labels: HashMap<String, String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigInspect {
    #[serde(rename = "ID", default, deserialize_with = "deserialize_null_default")]
    pub id: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub version: ObjectVersion,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub created_at: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub updated_at: String,
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub spec: SecretSpec,
}
