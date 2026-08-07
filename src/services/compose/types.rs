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

impl From<crate::db::models::compose_projects::ComposeProject> for ComposeRecord {
    fn from(item: crate::db::models::compose_projects::ComposeProject) -> Self {
        Self {
            id: item
                .id
                .as_deref()
                .and_then(|s| s.parse::<i64>().ok())
                .unwrap_or(0),
            name: item.name,
            app_name: item.app_name,
            description: item.description,
            env_var: item.env_var,
            compose_file: item.compose_file,
            source_type: item.source_type,
            compose_type: ComposeType::from_str(&item.compose_type)
                .unwrap_or(ComposeType::DockerCompose),
            compose_status: item.compose_status,
            trigger_type: item.trigger_type,
            repository: item.repository,
            owner: item.owner,
            branch: item.branch,
            gitlab_repository: item.gitlab_repository,
            gitlab_owner: item.gitlab_owner,
            gitlab_branch: item.gitlab_branch,
            gitea_repository: item.gitea_repository,
            gitea_owner: item.gitea_owner,
            gitea_branch: item.gitea_branch,
            bitbucket_repository: item.bitbucket_repository,
            bitbucket_owner: item.bitbucket_owner,
            bitbucket_branch: item.bitbucket_branch,
            custom_git_url: item.custom_git_url,
            custom_git_branch: item.custom_git_branch,
            command: item.command,
            compose_path: item.compose_path,
            service_networks: item.service_networks,
            environment_id: item.environment_id,
            server_id: item.server_id,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}
