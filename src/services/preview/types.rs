use serde::Serialize;

use crate::repository::PreviewDeploymentRow;

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct PreviewDeploymentView {
    pub id: i64,
    pub base_application_id: i64,
    pub preview_application_id: Option<i64>,
    pub provider_type: String,
    pub provider_id: i64,
    pub owner: String,
    pub repository: String,
    pub pull_request_number: String,
    pub source_branch: String,
    pub source_owner: Option<String>,
    pub source_repository: Option<String>,
    pub target_branch: String,
    pub commit_sha: Option<String>,
    pub author: Option<String>,
    pub status: String,
    pub domain: String,
    pub last_deployment_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<PreviewDeploymentRow> for PreviewDeploymentView {
    fn from(row: PreviewDeploymentRow) -> Self {
        Self {
            id: row.id,
            base_application_id: row.base_application_id,
            preview_application_id: row.preview_application_id,
            provider_type: row.provider_type,
            provider_id: row.provider_id,
            owner: row.owner,
            repository: row.repository,
            pull_request_number: row.pull_request_number,
            source_branch: row.source_branch,
            source_owner: row.source_owner,
            source_repository: row.source_repository,
            target_branch: row.target_branch,
            commit_sha: row.commit_sha,
            author: row.author,
            status: row.status,
            domain: row.domain,
            last_deployment_id: row.last_deployment_id,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PreviewLifecycleOutcome {
    pub created: usize,
    pub redeployed: usize,
    pub removed: usize,
    pub skipped_permission: usize,
    pub skipped_limit: usize,
    pub already_running: usize,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct PreviewDeploymentOutcome {
    pub preview: PreviewDeploymentView,
    pub deployment_id: Option<i64>,
}
