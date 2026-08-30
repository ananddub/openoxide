use serde::Serialize;

use crate::flash_gen::models::PreviewDeployments;

#[derive(Debug, Clone, Serialize, ts_rs::TS)]
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

macro_rules! impl_view_from_row {
    ($row:ty) => {
        impl From<$row> for PreviewDeploymentView {
            fn from(row: $row) -> Self {
                Self {
                    id: row.id,
                    base_application_id: row.base_service_id,
                    preview_application_id: row.preview_service_id,
                    provider_type: row.provider_type,
                    provider_id: row.provider_id,
                    owner: row.owner,
                    repository: row.repository,
                    pull_request_number: row.pull_request_number,
                    source_branch: row.source_branch,
                    source_owner: optional_field(row.source_owner),
                    source_repository: optional_field(row.source_repository),
                    target_branch: row.target_branch,
                    commit_sha: optional_field(row.commit_sha),
                    author: optional_field(row.author),
                    status: row.status,
                    domain: row.domain,
                    last_deployment_id: row.last_deployment_id,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                }
            }
        }
    };
}

impl_view_from_row!(PreviewDeployments);

/// The generated queries bind nullable TEXT columns as non-Option strings, so
/// a stored NULL round-trips as `Some("")`. Empty strings are folded back to
/// `None` to keep the public view shape identical to the previous repository.
pub(super) fn optional_field(value: Option<String>) -> Option<String> {
    match value {
        Some(value) if value.is_empty() => None,
        other => other,
    }
}

/// Base application (a `services` row in the unified schema) that matches an
/// incoming pull-request event and is configured for preview deployments.
#[derive(Debug, Clone)]
pub(super) struct PreviewTarget {
    pub base_service_id: i64,
    pub provider_id: i64,
    pub preview_limit: i64,
    pub require_collaborator_permissions: bool,
}

/// Payload used to insert a new `preview_deployments` row.
pub(super) struct NewPreviewDeployment<'a> {
    pub base_service_id: i64,
    pub preview_service_id: i64,
    pub provider_type: &'a str,
    pub provider_id: i64,
    pub owner: &'a str,
    pub repository: &'a str,
    pub pull_request_number: &'a str,
    pub source_branch: &'a str,
    pub source_owner: Option<&'a str>,
    pub source_repository: Option<&'a str>,
    pub target_branch: &'a str,
    pub commit_sha: Option<&'a str>,
    pub author: Option<&'a str>,
    pub domain: &'a str,
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

#[derive(Debug, Clone, Serialize)]
pub struct PreviewDeploymentOutcome {
    pub preview: PreviewDeploymentView,
    pub deployment_id: Option<i64>,
}
