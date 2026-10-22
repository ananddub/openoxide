use os::string_enum;

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ApplicationStatus {
        default = Idle;
        Idle => "IDLE",
        Queued => "QUEUED",
        Building => "BUILDING",
        Deploying => "DEPLOYING",
        Starting => "STARTING",
        Running => "RUNNING",
        Healthy => "HEALTHY",
        Stopping => "STOPPING",
        Stopped => "STOPPED",
        Cancelling => "CANCELLING",
        Cancelled => "CANCELLED",
        Done => "DONE",
        Failed => "FAILED",
        Error => "ERROR"
    }
}

impl ApplicationStatus {
    pub fn is_building(self) -> bool {
        matches!(
            self,
            Self::Queued | Self::Building | Self::Deploying | Self::Starting | Self::Cancelling
        )
    }
}
#[derive(Debug, Clone)]
pub struct ApplicationRecord {
    pub id: i64,
    pub name: String,
    pub app_name: String,
    pub description: Option<String>,
    pub source_type: String,
    pub build_type: String,
    // Build-config round-trip: the settings cards read these back after a
    // save (they live on service_application / service_git, not the proto
    // Application message until now).
    pub dockerfile: Option<String>,
    pub docker_context_path: Option<String>,
    pub docker_build_stage: Option<String>,
    pub publish_directory: Option<String>,
    pub is_static_spa: bool,
    pub railpack_version: Option<String>,
    pub heroku_version: Option<String>,
    pub build_path: Option<String>,
    pub drop_build_path: Option<String>,
    pub clean_cache: bool,
    pub auto_deploy: bool,
    pub app_status: String,
    pub last_error: Option<String>,
    pub trigger_type: String,
    pub environment_id: i64,
    pub server_id: Option<i64>,
    pub build_server_id: Option<i64>,
    pub registry_id: Option<i64>,
    pub network_ids: String,
    pub detach_rustploy_network: i64,
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
    pub preview_env: Option<String>,
    pub preview_build_args: Option<String>,
    pub preview_build_secrets: Option<String>,
    pub preview_labels: Option<String>,
    pub preview_wildcard: Option<String>,
    pub preview_port: Option<i64>,
    pub preview_https: bool,
    pub preview_path: Option<String>,
    pub preview_certificate_type: String,
    pub preview_custom_cert_resolver: Option<String>,
    pub preview_limit: Option<i64>,
    pub previews_active: bool,
    pub preview_require_collaborator_permissions: bool,
    pub memory_reservation: Option<String>,
    pub memory_limit: Option<String>,
    pub cpu_reservation: Option<String>,
    pub cpu_limit: Option<String>,
    pub replicas: Option<i64>,
    pub command: Option<String>,
    pub args: Option<String>,
    pub watch_paths: Option<String>,
    pub enable_submodules: bool,
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
    Stop,
}

impl ApplicationOperation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Deploy => "deploy",
            Self::Redeploy => "redeploy",
            Self::Rebuild => "rebuild",
            Self::Reload => "reload",
            Self::Start => "start",
            Self::Stop => "stop",
        }
    }

    pub(super) fn title(self) -> &'static str {
        match self {
            Self::Deploy => "Application deploy",
            Self::Redeploy => "Application redeploy",
            Self::Rebuild => "Application rebuild",
            Self::Reload => "Application reload",
            Self::Start => "Application start",
            Self::Stop => "Application stop",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ApplicationOperationResult {
    pub application: ApplicationRecord,
    pub deployment_id: Option<i64>,
    pub operation: ApplicationOperation,
}

impl
    From<(
        crate::flash_gen::models::Services,
        crate::flash_gen::models::ServiceApplication,
        Option<crate::flash_gen::models::ServiceGit>,
    )> for ApplicationRecord
{
    fn from(
        (service, ext, git): (
            crate::flash_gen::models::Services,
            crate::flash_gen::models::ServiceApplication,
            Option<crate::flash_gen::models::ServiceGit>,
        ),
    ) -> Self {
        let git = git.as_ref();
        // Source type resolution mirrors ApplicationGetSpecRow: the
        // service_git row wins, otherwise DOCKER/DROP are derived from
        // docker_image / drop_build_path.
        let source_type = git.map(|g| g.source_type.clone()).unwrap_or_else(|| {
            if service.docker_image.is_some() {
                "DOCKER".into()
            } else if ext.drop_build_path.is_some() {
                "DROP".into()
            } else {
                "NONE".into()
            }
        });
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
            source_type,
            dockerfile: normalize(ext.dockerfile),
            docker_context_path: normalize(ext.docker_context_path),
            docker_build_stage: normalize(ext.docker_build_stage),
            publish_directory: normalize(ext.publish_directory),
            is_static_spa: ext.is_static_spa.unwrap_or(0) != 0,
            railpack_version: normalize(ext.railpack_version),
            heroku_version: normalize(ext.heroku_version),
            build_path: normalize(ext.build_path),
            drop_build_path: normalize(ext.drop_build_path),
            clean_cache: ext.clean_cache != 0,
            auto_deploy: git
                .map(|g| g.auto_deploy.unwrap_or(1) != 0)
                .unwrap_or(false),
            build_type: ext.build_type,
            app_status: service.app_status,
            last_error: service.last_error,
            trigger_type: git
                .map(|g| g.trigger_type.clone())
                .unwrap_or_else(|| "PUSH".into()),
            environment_id: service.environment_id,
            server_id: service.server_id,
            build_server_id: ext.build_server_id,
            registry_id: ext.registry_id,
            network_ids: service.network_ids,
            detach_rustploy_network: service.detach_rustploy_network,
            env_var: normalize(service.env_var),
            icon: normalize(service.icon),
            repository: git.and_then(|g| g.repository.clone()),
            owner: git.and_then(|g| g.owner.clone()),
            branch: git.and_then(|g| g.branch.clone()),
            gitlab_repository: git.and_then(|g| g.gitlab_repository.clone()),
            gitlab_owner: git.and_then(|g| g.gitlab_owner.clone()),
            gitlab_branch: git.and_then(|g| g.gitlab_branch.clone()),
            gitea_repository: git.and_then(|g| g.gitea_repository.clone()),
            gitea_owner: git.and_then(|g| g.gitea_owner.clone()),
            gitea_branch: git.and_then(|g| g.gitea_branch.clone()),
            bitbucket_repository: git.and_then(|g| g.bitbucket_repository.clone()),
            bitbucket_owner: git.and_then(|g| g.bitbucket_owner.clone()),
            bitbucket_branch: git.and_then(|g| g.bitbucket_branch.clone()),
            docker_image: service.docker_image,
            registry_url: ext.registry_url,
            custom_git_url: git.and_then(|g| g.custom_git_url.clone()),
            custom_git_branch: git.and_then(|g| g.custom_git_branch.clone()),
            preview_env: ext.preview_env,
            preview_build_args: ext.preview_build_args,
            preview_build_secrets: ext.preview_build_secrets,
            preview_labels: ext.preview_labels,
            preview_wildcard: ext.preview_wildcard,
            preview_port: ext.preview_port,
            preview_https: ext.preview_https != 0,
            preview_path: ext.preview_path,
            preview_certificate_type: ext.preview_certificate_type,
            preview_custom_cert_resolver: ext.preview_custom_cert_resolver,
            preview_limit: ext.preview_limit,
            previews_active: ext.is_preview_deployments_active != 0,
            preview_require_collaborator_permissions: ext.preview_require_collaborator_permissions
                != 0,
            memory_reservation: service.memory_reservation,
            memory_limit: service.memory_limit,
            cpu_reservation: service.cpu_reservation,
            cpu_limit: service.cpu_limit,
            replicas: Some(service.replicas),
            command: normalize(service.command),
            args: normalize(service.args),
            watch_paths: git.and_then(|g| normalize(g.watch_paths.clone())),
            enable_submodules: git.map(|g| g.enable_submodules != 0).unwrap_or(false),
            created_at: service.created_at,
            updated_at: service.updated_at,
        }
    }
}
