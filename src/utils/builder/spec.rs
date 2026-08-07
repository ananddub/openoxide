use os::string_enum;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum SourceType {
        default = Docker;

        Docker => "DOCKER",
        Git => "GIT",
        Github => "GITHUB",
        Gitlab => "GITLAB",
        Bitbucket => "BITBUCKET",
        Gitea => "GITEA",
        Raw => "RAW",
        Drop => "DROP",
    }
}

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum BuildType {
        default = Dockerfile;

        Dockerfile => "DOCKERFILE",
        Nixpacks => "NIXPACKS",
        Paketo => "PAKETO_BUILDPACKS",
        Heroku => "HEROKU",
        Railpack => "RAILPACK",
        Static => "STATIC",
    }
}

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum MountType {
        default = Bind;

        Volume => "VOLUME",
        Bind => "BIND",
        File => "FILE",
    }
}

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum RuntimeType {
        default = Compose;

        Compose => "DOCKER-COMPOSE",
        Stack => "STACK",
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SourceSpec {
    Docker {
        image: String,
        registry: Option<RegistryAuth>,
    },
    Git {
        url: String,
        branch: Option<String>,
        submodules: bool,
        protocol: crate::utils::provider::CloneProtocol,
        auth: Option<crate::utils::git::types::GitAuth>,
    },
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegistryAuth {
    pub registry: String,
    pub username: String,
    pub password: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BuildStrategy {
    Dockerfile {
        dockerfile: String,
        context: String,
        target: Option<String>,
        no_cache: bool,
    },
    Nixpacks,
    Paketo,
    Heroku,
    Railpack {
        version: String,
    },
    Static {
        publish_directory: String,
        spa: bool,
    },
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ResourceSpec {
    pub memory_limit: Option<String>,
    pub memory_reservation: Option<String>,
    pub cpu_limit: Option<String>,
    pub cpu_reservation: Option<String>,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct HealthSpec {
    pub command: Vec<String>,
    pub interval: Option<String>,
    pub timeout: Option<String>,
    pub retries: Option<u32>,
    pub start_period: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MountKind {
    Bind,
    Volume,
    File,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MountSpec {
    #[serde(rename = "type")]
    pub kind: MountKind,
    pub source: String,
    pub target: String,
    #[serde(default)]
    pub read_only: bool,
    #[serde(skip)]
    pub content: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatchSpec {
    pub patch_type: String,
    pub file_path: String,
    pub content: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PortSpec {
    pub published: u16,
    pub target: u16,
    pub protocol: String,
    pub mode: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RedirectSpec {
    pub key: String,
    pub regex: String,
    pub replacement: String,
    pub permanent: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BasicAuthSpec {
    pub username: String,
    pub password_hash: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StructuredMiddlewareSpec {
    Compress {
        name: String,
    },
    Headers {
        name: String,
        headers: BTreeMap<String, String>,
    },
    RateLimit {
        name: String,
        average: i64,
        burst: i64,
    },
    IpAllowList {
        name: String,
        source_ranges: Vec<String>,
    },
}

impl StructuredMiddlewareSpec {
    pub fn name(&self) -> &str {
        match self {
            Self::Compress { name }
            | Self::Headers { name, .. }
            | Self::RateLimit { name, .. }
            | Self::IpAllowList { name, .. } => name,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DomainSpec {
    pub key: String,
    pub host: String,
    pub https: bool,
    pub port: u16,
    pub service_name: Option<String>,
    pub path: String,
    pub internal_path: String,
    pub strip_path: bool,
    pub entrypoint: Option<String>,
    pub certificate_type: String,
    pub custom_cert_resolver: Option<String>,
    pub middlewares: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApplicationSpec {
    pub app_name: String,
    pub stack_name: String,
    pub source: SourceSpec,
    pub build: Option<BuildStrategy>,
    pub work_directory: String,
    pub image: String,
    pub environment: BTreeMap<String, String>,
    pub build_args: BTreeMap<String, String>,
    pub build_secrets: BTreeMap<String, String>,
    pub command: Option<Vec<String>>,
    pub args: Vec<String>,
    pub replicas: u32,
    pub networks: Vec<String>,
    pub mounts: Vec<MountSpec>,
    pub patches: Vec<PatchSpec>,
    pub ports: Vec<PortSpec>,
    pub redirects: Vec<RedirectSpec>,
    pub basic_auth: Vec<BasicAuthSpec>,
    pub middlewares: Vec<StructuredMiddlewareSpec>,
    pub domains: Vec<DomainSpec>,
    pub resources: ResourceSpec,
    pub healthcheck: Option<HealthSpec>,
    pub placement_constraints: Vec<String>,
    pub stop_grace_period: Option<String>,
}

impl ApplicationSpec {
    pub fn service_name(&self) -> String {
        format!("{}_{}", self.stack_name, self.app_name)
    }
}

#[derive(Clone, Debug)]
pub struct DeploymentResult {
    pub app_name: String,
    pub image: String,
    pub service_name: String,
    pub stack_file: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BuilderEvent {
    RecoverAfterRestart,
    Preparing,
    SourceReady,
    Building,
    ImageReady,
    Deploying,
    Routing,
    HealthCheck,
    Deployed,
    Cancelled,
    Message(String),
    Failed(String),
}
