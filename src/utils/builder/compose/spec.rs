use crate::utils::builder::spec::{DomainSpec, MountSpec, PatchSpec};
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub enum ComposeSource {
    Raw {
        content: String,
    },
    Git {
        url: String,
        branch: String,
        submodules: bool,
        protocol: crate::utils::provider::CloneProtocol,
        auth: Option<crate::utils::git::types::GitAuth>,
    },
    Drop,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ComposeRuntime {
    Compose,
    Stack,
}

#[derive(Clone, Debug)]
pub struct ComposeServiceNetworkSpec {
    pub service_name: String,
    pub networks: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct ComposeSpec {
    pub app_name: String,
    pub stack_name: String,
    pub source: ComposeSource,
    pub runtime: ComposeRuntime,
    pub work_directory: String,
    pub compose_path: String,
    pub rendered_stack_file: String,
    pub env_file: String,
    pub environment: BTreeMap<String, String>,
    pub mounts: Vec<MountSpec>,
    pub patches: Vec<PatchSpec>,
    pub domains: Vec<DomainSpec>,
    pub service_networks: Vec<ComposeServiceNetworkSpec>,
    pub randomize: bool,
    pub suffix: String,
    pub isolated_deployment: bool,
    pub isolated_deployments_volume: bool,
}

impl ComposeSpec {
    pub fn compose_file_path(&self) -> String {
        match self.source {
            ComposeSource::Raw { .. } => self.compose_path.clone(),
            ComposeSource::Git { .. } | ComposeSource::Drop => format!(
                "{}/{}",
                self.work_directory.trim_end_matches('/'),
                self.compose_path
                    .trim_start_matches("./")
                    .trim_start_matches('/')
            ),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ComposeDeploymentResult {
    pub app_name: String,
    pub stack_name: String,
    pub compose_file: String,
}
