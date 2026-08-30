use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::ResourceResponse;

use serde::{Deserialize, Serialize};

pub fn map_db_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("tag not found"),
        other => {
            tracing::error!(error = %other, "tag database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}

pub fn tag_to_resource(tag_dto: TagDto) -> ResourceResponse {
    let payload = serde_json::to_string(&tag_dto).unwrap_or_default();
    ResourceResponse {
        id: tag_dto.id,
        kind: "tag".into(),
        payload,
        ..Default::default()
    }
}

// ── Helper DTOs for ResourceRequest payloads ────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagDto {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub organization_id: i64,
    pub created_at: i64,
}

impl From<crate::flash_gen::models::Tags> for TagDto {
    fn from(t: crate::flash_gen::models::Tags) -> Self {
        Self {
            id: t.id,
            name: t.name,
            color: t.color,
            organization_id: t.organization_id,
            created_at: t.created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachProjectTagDto {
    pub tag_id: i64,
}
