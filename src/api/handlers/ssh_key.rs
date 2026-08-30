use std::sync::Arc;

use auto_di::singleton;
use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, OperationResult, ResourceListResponse, ResourceQuery, ResourceRequest, ResourceResponse,
    SshKeyService, SshKeyServiceExt,
};
use crate::services::ssh_key::{
    CreateSshKeyDto, PatchSshKeyDto, SshKeyService as AppSshKeyService,
};

pub struct SshKeyController {
    service: Arc<AppSshKeyService>,
}

#[singleton]
impl SshKeyController {
    pub fn new(service: Arc<AppSshKeyService>) -> Self {
        Self { service }
    }
}

fn map_db_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("ssh key not found"),
        sqlx::Error::Protocol(msg) => ConnectError::invalid_argument(msg.clone()),
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        sqlx::Error::Database(e) if e.is_foreign_key_violation() => {
            ConnectError::new(ErrorCode::FailedPrecondition,
                "Cannot delete SSH Key: It is currently assigned to active servers or git repositories.")
        }
        other => {
            tracing::error!(error = %other, "ssh key database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}

fn key_to_resource(dto: SshKeyResponseDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id,
        kind: "ssh_key".into(),
        payload,
        ..Default::default()
    }
}

#[auto_grpc::service(connect = true, auth = true)]
impl SshKeyService for SshKeyController {
    #[permission(Server, CanRead)]
    async fn list_keys<'a>(
        &'a self,
        ctx: RequestContext,
        _query: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let items = self.service.list().await.map_err(map_db_error)?;
        let items = items.into_iter().map(SshKeyResponseDto::from).map(key_to_resource).collect();
        Response::ok(ResourceListResponse { items, ..Default::default() })
    }

    #[permission(Server, CanRead)]
    async fn get_key<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let key = self.service.get_by_id(request.id).await.map_err(map_db_error)?;
        Response::ok(key_to_resource(SshKeyResponseDto::from(key)))
    }

    #[permission(Server, CanCreate)]
    async fn create_key<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: CreateSshKeyDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let key = self.service.create(dto).await.map_err(map_db_error)?;
        Response::ok(key_to_resource(SshKeyResponseDto::from(key)))
    }

    #[permission(Server, CanCreate)]
    async fn generate_key<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: GenerateSshKeyDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let key = self.service.generate(dto.name, dto.description, &dto.key_type)
            .await.map_err(map_db_error)?;
        Response::ok(key_to_resource(SshKeyResponseDto::from(key)))
    }

    async fn generate_pair<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: GeneratePairRequestDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let (private_key, public_key) = crate::utils::ssh::generate_keypair(&dto.key_type)
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?;
        let resp = GeneratePairResponseDto { private_key, public_key };
        let payload = serde_json::to_string(&resp).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: 0,
            kind: "ssh_key_pair".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Server, CanCreate)]
    async fn update_key<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: PatchSshKeyDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let key = self.service.patch(request.id, dto).await.map_err(map_db_error)?;
        Response::ok(key_to_resource(SshKeyResponseDto::from(key)))
    }

    #[permission(Server, CanCreate)]
    async fn mark_used<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        self.service.mark_used(request.id).await.map_err(map_db_error)?;
        Response::ok(OperationResult {
            success: true,
            operation: "mark_used".into(),
            ..Default::default()
        })
    }

    #[permission(Server, CanDelete)]
    async fn delete_key<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.delete(request.id).await.map_err(map_db_error)?;
        Response::ok(Empty::default())
    }
}

// ── Helper DTOs for deserializing ResourceRequest payloads ────────────

use crate::db::models::ssh_keys::SshKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct GenerateSshKeyDto {
    name: String,
    description: Option<String>,
    key_type: String, // "ed25519" or "rsa"
}

#[derive(Debug, Deserialize)]
struct GeneratePairRequestDto {
    #[serde(default = "default_key_type")]
    key_type: String, // "ed25519" or "rsa"
}

fn default_key_type() -> String {
    "ed25519".into()
}

#[derive(Debug, Clone, Serialize)]
struct GeneratePairResponseDto {
    private_key: String,
    public_key: String,
}

#[derive(Debug, Clone, Serialize)]
struct SshKeyResponseDto {
    id: i64,
    name: String,
    description: Option<String>,
    public_key: String,
    private_key: Option<String>,
    has_private_key: bool,
    last_used_at: Option<i64>,
    created_at: i64,
    updated_at: i64,
}

impl From<SshKey> for SshKeyResponseDto {
    fn from(value: SshKey) -> Self {
        let has_priv = !value.private_key.is_empty();
        let priv_key = if has_priv {
            Some(value.private_key)
        } else {
            None
        };
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            public_key: value.public_key,
            private_key: priv_key,
            has_private_key: has_priv,
            last_used_at: value.last_used_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
