use os::string_enum;

string_enum! {
       #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum ComposeType {
                default = Stack;
                DockerCompose => "DOCKER-COMPOSE",
                Stack => "STACK"
        }
}

string_enum! {
       #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum ComposeStatus {
                default = Idle;
                Idle => "IDLE",
                Queued => "QUEUED",
                Starting => "STARTING",
                Running => "RUNNING",
                Stopping => "STOPPING",
                Stopped => "STOPPED",
                Cancelling => "CANCELLING",
                Cancelled => "CANCELLED",
                Done => "DONE",
                Error => "ERROR"
        }
}

impl ComposeStatus {
    pub fn is_building(self) -> bool {
        matches!(self, Self::Queued | Self::Starting | Self::Cancelling)
    }
}

// impl From<String> for ComposeType {
//     fn from(value: String) -> Self {
//         Self::from_str(&value).unwrap_or_else(|| {
//             tracing::warn!(compose_type = %value, "unknown compose_type, defaulting to DockerCompose");
//             Self::DockerCompose
//         })
//     }
// }

#[derive(Debug, Clone)]
pub struct ComposeRecord {
    pub id: i64,
    pub name: String,
    pub app_name: String,
    pub description: Option<String>,
    pub env_var: Option<String>,
    pub compose_file: String,
    pub source_type: String,
    pub compose_type: ComposeType,
    pub compose_status: String,
    pub last_error: Option<String>,
    pub trigger_type: String,
    pub repository: Option<String>,
    pub owner: Option<String>,
    pub branch: Option<String>,
    pub gitlab_repository: Option<String>,
    pub gitlab_owner: Option<String>,
    pub gitlab_branch: Option<String>,
    pub gitea_repository: Option<String>,
    pub gitea_owner: Option<String>,
    pub gitea_branch: Option<String>,
    pub bitbucket_repository: Option<String>,
    pub bitbucket_owner: Option<String>,
    pub bitbucket_branch: Option<String>,
    pub custom_git_url: Option<String>,
    pub custom_git_branch: Option<String>,
    pub command: String,
    pub compose_path: String,
    pub service_networks: String,
    pub environment_id: i64,
    pub server_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Copy)]
pub enum ComposeOperation {
    Deploy,
    Redeploy,
    Reload,
    Start,
    Stop,
}

impl ComposeOperation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Deploy => "deploy",
            Self::Redeploy => "redeploy",
            Self::Reload => "reload",
            Self::Start => "start",
            Self::Stop => "stop",
        }
    }

    pub(super) fn title(self) -> &'static str {
        match self {
            Self::Deploy => "Compose deploy",
            Self::Redeploy => "Compose redeploy",
            Self::Reload => "Compose reload",
            Self::Start => "Compose start",
            Self::Stop => "Compose stop",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ComposeOperationResult {
    pub compose: ComposeRecord,
    pub deployment_id: Option<i64>,
    pub operation: ComposeOperation,
}

impl From<(crate::flash_gen::models::Services, crate::flash_gen::models::ServiceCompose, Option<crate::flash_gen::models::ServiceGit>)>
    for ComposeRecord
{
    fn from(
        (service, ext, git): (
            crate::flash_gen::models::Services,
            crate::flash_gen::models::ServiceCompose,
            Option<crate::flash_gen::models::ServiceGit>,
        ),
    ) -> Self {
        let git = git.as_ref();
        // Flash bind params for nullable TEXT columns are non-Option, so
        // empty strings round-trip where the old table stored NULL.
        let normalize = |value: Option<String>| match value {
            Some(text) if text.is_empty() => None,
            other => other,
        };
        Self {
            id: service.id,
            name: service.name,
            app_name: service.app_name,
            description: normalize(service.description),
            env_var: normalize(service.env_var),
            compose_file: ext.compose_file,
            source_type: git
                .map(|g| g.source_type.clone())
                .unwrap_or_else(|| "RAW".into()),
            compose_type: ComposeType::from_str(&ext.compose_type)
                .unwrap_or(ComposeType::DockerCompose),
            compose_status: service.app_status,
            last_error: service.last_error,
            trigger_type: git
                .map(|g| g.trigger_type.clone())
                .unwrap_or_else(|| "PUSH".into()),
            repository: git.and_then(|g| normalize(g.repository.clone())),
            owner: git.and_then(|g| normalize(g.owner.clone())),
            branch: git.and_then(|g| normalize(g.branch.clone())),
            gitlab_repository: git.and_then(|g| normalize(g.gitlab_repository.clone())),
            gitlab_owner: git.and_then(|g| normalize(g.gitlab_owner.clone())),
            gitlab_branch: git.and_then(|g| normalize(g.gitlab_branch.clone())),
            gitea_repository: git.and_then(|g| normalize(g.gitea_repository.clone())),
            gitea_owner: git.and_then(|g| normalize(g.gitea_owner.clone())),
            gitea_branch: git.and_then(|g| normalize(g.gitea_branch.clone())),
            bitbucket_repository: git.and_then(|g| normalize(g.bitbucket_repository.clone())),
            bitbucket_owner: git.and_then(|g| normalize(g.bitbucket_owner.clone())),
            bitbucket_branch: git.and_then(|g| normalize(g.bitbucket_branch.clone())),
            custom_git_url: git.and_then(|g| normalize(g.custom_git_url.clone())),
            custom_git_branch: git.and_then(|g| normalize(g.custom_git_branch.clone())),
            command: service.command.unwrap_or_default(),
            compose_path: ext.compose_path,
            service_networks: ext.service_networks,
            environment_id: service.environment_id,
            server_id: service.server_id,
            created_at: service.created_at,
            updated_at: service.updated_at,
        }
    }
}
