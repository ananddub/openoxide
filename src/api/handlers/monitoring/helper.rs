use connectrpc::{ConnectError, ErrorCode};
use serde::{Deserialize, Serialize};

use crate::services::observability::monitoring::agent_auth::MonitoringAgentAuth;

#[derive(Clone, Debug, Deserialize)]
pub struct ContainerQueryParam {
    #[serde(rename = "appName")]
    pub app_name: Option<String>,
    pub limit: Option<i64>,
    pub organization_id: i64,
}

pub fn internal(error: sqlx::Error) -> ConnectError {
    tracing::error!(error = %error, "monitoring operation failed");
    ConnectError::new(ErrorCode::Internal, error.to_string())
}

pub async fn verify_server_organization(
    auth: &MonitoringAgentAuth,
    server_id: i64,
    organization_id: i64,
) -> Result<(), ConnectError> {
    let bound = auth
        .organization_id(server_id)
        .await
        .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?;
    if bound == Some(organization_id) {
        Ok(())
    } else {
        Err(ConnectError::not_found("monitoring server not found"))
    }
}

pub fn verify_claim_organization(claim_organization_id: i64, organization_id: i64) -> Result<(), ConnectError> {
    if claim_organization_id == organization_id {
        Ok(())
    } else {
        Err(ConnectError::new(ErrorCode::PermissionDenied, "organization does not match authenticated scope"))
    }
}

// ── Monitoring payload DTOs ───────────────────────────────────────

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MonitoringStatusResponseDto {
    pub status: String,
    pub service: String,
    pub endpoints: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RotateMonitoringTokenDto {
    pub organization_id: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MonitoringTokenResponseDto {
    pub server_id: i64,
    pub organization_id: i64,
    pub token: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MonitoringAgentStatusDto {
    pub server_id: i64,
    pub organization_id: i64,
    pub last_seen_at: Option<i64>,
    pub agent_version: Option<String>,
    pub state: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateMaintenanceWindowDto {
    pub server_id: Option<i64>,
    pub starts_at: i64,
    pub ends_at: i64,
    pub reason: String,
}
