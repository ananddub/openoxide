#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CancelDeploymentResult {
    CancelRequested,
    NotRunning,
    NotCancellable,
    NotActiveInThisProcess,
}

#[derive(Debug, Clone, Default)]
pub struct DeploymentListFilter {
    pub status: Option<String>,
    pub state: Option<String>,
    pub application_id: Option<i64>,
    pub compose_id: Option<i64>,
    pub database_id: Option<i64>,
    pub server_id: Option<i64>,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Clone, Default)]
pub struct DockerLogOptions {
    pub tail: usize,
    pub timestamps: bool,
    pub follow: bool,
    pub since: Option<String>,
    pub until: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ComposeLogOptions {
    pub file: Option<String>,
    pub project_directory: Option<String>,
    pub project_name: Option<String>,
    pub service: Option<String>,
    pub logs: DockerLogOptions,
}
