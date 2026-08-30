use connectrpc::{ConnectError, ErrorCode};
use serde::{Deserialize, Serialize};

use crate::api::connect::generated::openoxide::api::v1::ResourceResponse;
use crate::services::integrations::git::{GitProviderConfigView, GitProviderView};

pub fn map_db_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("git provider not found"),
        sqlx::Error::Protocol(msg) if msg.contains("assigned") => {
            ConnectError::new(ErrorCode::AlreadyExists, msg.clone())
        }
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        other => {
            tracing::error!(error = %other, "git provider operation failed");
            ConnectError::new(ErrorCode::Internal, "git provider operation failed")
        }
    }
}

pub fn provider_error(err: String) -> ConnectError {
    let normalized = err.to_ascii_lowercase();
    if normalized.contains("not found") {
        ConnectError::not_found(err)
    } else if normalized.contains("required")
        || normalized.contains("invalid")
        || normalized.contains("unsupported")
        || normalized.contains("must end with")
        || normalized.contains("does not support")
    {
        ConnectError::invalid_argument(err)
    } else {
        ConnectError::new(ErrorCode::Unavailable, err)
    }
}

pub fn provider_to_resource(dto: GitProviderResponseDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id,
        kind: "git_provider".into(),
        payload,
        ..Default::default()
    }
}

// ── Helper DTOs for deserializing ResourceRequest payloads ────────────

#[derive(Debug, Deserialize)]
pub struct ProviderBaseDto {
    pub name: String,
    pub shared: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateGithubProviderDto {
    pub provider: ProviderBaseDto,
    pub app_name: Option<String>,
    pub app_id: Option<i64>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub installation_id: Option<String>,
    pub private_key: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateGitlabProviderDto {
    pub provider: ProviderBaseDto,
    pub url: String,
    pub internal_url: Option<String>,
    pub application_id: Option<String>,
    pub redirect_uri: Option<String>,
    pub secret: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub group_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateGiteaProviderDto {
    pub provider: ProviderBaseDto,
    pub url: String,
    pub internal_url: Option<String>,
    pub redirect_uri: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub scopes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBitbucketProviderDto {
    pub provider: ProviderBaseDto,
    pub username: Option<String>,
    pub email: Option<String>,
    pub app_password: Option<String>,
    pub api_token: Option<String>,
    pub workspace: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GitProviderResponseDto {
    pub id: i64,
    pub name: String,
    pub provider_type: String,
    pub shared: bool,
    pub configured: bool,
    pub webhook_configured: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub config: GitProviderConfigDto,
}

#[derive(Debug, Clone, Serialize)]
pub struct GitProviderConfigDto {
    pub url: Option<String>,
    pub internal_url: Option<String>,
    pub app_name: Option<String>,
    pub app_id: Option<i64>,
    pub client_id: Option<String>,
    pub installation_id: Option<String>,
    pub application_id: Option<String>,
    pub redirect_uri: Option<String>,
    pub group_name: Option<String>,
    pub scopes: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub workspace: Option<String>,
    pub has_client_secret: bool,
    pub has_private_key: bool,
    pub has_access_token: bool,
    pub has_app_password: bool,
    pub has_api_token: bool,
}

impl From<GitProviderConfigView> for GitProviderConfigDto {
    fn from(v: GitProviderConfigView) -> Self {
        Self {
            url: v.url,
            internal_url: v.internal_url,
            app_name: v.app_name,
            app_id: v.app_id,
            client_id: v.client_id,
            installation_id: v.installation_id,
            application_id: v.application_id,
            redirect_uri: v.redirect_uri,
            group_name: v.group_name,
            scopes: v.scopes,
            username: v.username,
            email: v.email,
            workspace: v.workspace,
            has_client_secret: v.has_client_secret,
            has_private_key: v.has_private_key,
            has_access_token: v.has_access_token,
            has_app_password: v.has_app_password,
            has_api_token: v.has_api_token,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CreatedGitProviderResponseDto {
    pub provider: GitProviderResponseDto,
    pub webhook_secret: String,
}

#[derive(Debug, Serialize)]
pub struct WebhookSecretResponseDto {
    pub webhook_secret: String,
}

#[derive(Debug, Deserialize)]
pub struct RepositoryReferenceQueryDto {
    pub owner: String,
    pub repository: String,
}

#[derive(Debug, Deserialize)]
pub struct WebhookRepositoryDto {
    pub owner: String,
    pub repository: String,
    pub callback_url: String,
}

#[derive(Debug, Deserialize)]
pub struct OAuthCallbackDto {
    pub code: Option<String>,
    pub state: String,
    pub installation_id: Option<String>,
    pub setup_action: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CollaboratorPermissionQueryDto {
    pub owner: String,
    pub repository: String,
    pub username: String,
}

impl From<GitProviderView> for GitProviderResponseDto {
    fn from(value: GitProviderView) -> Self {
        Self {
            id: value.id,
            name: value.name,
            provider_type: value.provider_type,
            shared: value.shared,
            configured: value.configured,
            webhook_configured: value.webhook_configured,
            created_at: value.created_at,
            updated_at: value.updated_at,
            config: value.config.into(),
        }
    }
}
