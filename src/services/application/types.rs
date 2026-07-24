#[derive(Debug, Clone)]
pub struct ApplicationRecord {
    pub id: i64,
    pub name: String,
    pub app_name: String,
    pub description: Option<String>,
    pub source_type: String,
    pub build_type: String,
    pub app_status: String,
    pub trigger_type: String,
    pub environment_id: i64,
    pub server_id: Option<i64>,
    pub build_server_id: Option<i64>,
    pub registry_id: Option<i64>,
    pub env_var: Option<String>,
    pub icon: Option<String>,
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
    pub docker_image: Option<String>,
    pub registry_url: Option<String>,
    pub custom_git_url: Option<String>,
    pub custom_git_branch: Option<String>,
    pub memory_reservation: Option<String>,
    pub memory_limit: Option<String>,
    pub cpu_reservation: Option<String>,
    pub cpu_limit: Option<String>,
    pub replicas: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Copy)]
pub enum ApplicationOperation {
    Deploy,
    Redeploy,
    Rebuild,
    Reload,
    Start,
}

impl ApplicationOperation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Deploy => "deploy",
            Self::Redeploy => "redeploy",
            Self::Rebuild => "rebuild",
            Self::Reload => "reload",
            Self::Start => "start",
        }
    }

    pub(super) fn title(self) -> &'static str {
        match self {
            Self::Deploy => "Application deploy",
            Self::Redeploy => "Application redeploy",
            Self::Rebuild => "Application rebuild",
            Self::Reload => "Application reload",
            Self::Start => "Application start",
        }
    }

    pub(super) fn target_status(self) -> &'static str {
        match self {
            Self::Deploy | Self::Redeploy | Self::Rebuild | Self::Reload | Self::Start => "RUNNING",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ApplicationOperationResult {
    pub application: ApplicationRecord,
    pub deployment_id: Option<i64>,
    pub operation: ApplicationOperation,
}

impl From<crate::db::models::applications::Application> for ApplicationRecord {
    fn from(app: crate::db::models::applications::Application) -> Self {
        Self {
            id: app
                .id
                .as_deref()
                .and_then(|s| s.parse::<i64>().ok())
                .unwrap_or(0),
            name: app.name,
            app_name: app.app_name,
            description: app.description,
            source_type: app.source_type,
            build_type: app.build_type,
            app_status: app.app_status,
            trigger_type: app.trigger_type,
            environment_id: app.environment_id,
            server_id: app.server_id,
            build_server_id: app.build_server_id,
            registry_id: app.registry_id,
            env_var: app.env_var,
            icon: app.icon,
            repository: app.repository,
            owner: app.owner,
            branch: app.branch,
            gitlab_repository: app.gitlab_repository,
            gitlab_owner: app.gitlab_owner,
            gitlab_branch: app.gitlab_branch,
            gitea_repository: app.gitea_repository,
            gitea_owner: app.gitea_owner,
            gitea_branch: app.gitea_branch,
            bitbucket_repository: app.bitbucket_repository,
            bitbucket_owner: app.bitbucket_owner,
            bitbucket_branch: app.bitbucket_branch,
            docker_image: app.docker_image,
            registry_url: app.registry_url,
            custom_git_url: app.custom_git_url,
            custom_git_branch: app.custom_git_branch,
            memory_reservation: app.memory_reservation,
            memory_limit: app.memory_limit,
            cpu_reservation: app.cpu_reservation,
            cpu_limit: app.cpu_limit,
            replicas: Some(app.replicas),
            created_at: app.created_at,
            updated_at: app.updated_at,
        }
    }
}
