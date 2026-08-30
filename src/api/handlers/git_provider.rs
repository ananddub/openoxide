use std::sync::Arc;

use auto_di::singleton;
use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, GitProviderService, GitProviderServiceExt, OperationResult, ResourceListResponse,
    ResourceQuery, ResourceRequest, ResourceResponse,
};
use crate::services::git_provider::{
    CreateProvider, GitProviderService as AppGitProviderService, ProviderCredentials,
    UpdateProvider,
};

pub struct GitProviderController {
    service: Arc<AppGitProviderService>,
}

#[singleton]
impl GitProviderController {
    pub fn new(service: Arc<AppGitProviderService>) -> Self {
        Self { service }
    }
}

fn map_db_error(err: sqlx::Error) -> ConnectError {
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

fn provider_error(err: String) -> ConnectError {
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

fn provider_to_resource(dto: GitProviderResponseDto) -> ResourceResponse {
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id,
        kind: "git_provider".into(),
        payload,
        ..Default::default()
    }
}

#[auto_grpc::service(connect = true, auth = true)]
impl GitProviderService for GitProviderController {
    #[permission(Application, CanRead)]
    async fn list_git_providers<'a>(
        &'a self,
        ctx: RequestContext,
        _query: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let items = self.service.list().await.map_err(map_db_error)?;
        let items = items
            .into_iter()
            .map(GitProviderResponseDto::from)
            .map(provider_to_resource)
            .collect();
        Response::ok(ResourceListResponse {
            items,
            ..Default::default()
        })
    }

    #[permission(Application, CanRead)]
    async fn get_git_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let provider = self.service.get(request.id).await.map_err(map_db_error)?;
        Response::ok(provider_to_resource(GitProviderResponseDto::from(provider)))
    }

    #[permission(Application, CanCreate)]
    async fn create_github_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: CreateGithubProviderDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let (provider, webhook_secret) = self
            .service
            .create(CreateProvider {
                name: dto.provider.name,
                shared: dto.provider.shared.unwrap_or(true),
                credentials: ProviderCredentials::Github {
                    app_name: dto.app_name,
                    app_id: dto.app_id,
                    client_id: dto.client_id,
                    client_secret: dto.client_secret,
                    installation_id: dto.installation_id,
                    private_key: dto.private_key,
                },
            })
            .await
            .map_err(map_db_error)?;
        let created = CreatedGitProviderResponseDto {
            provider: provider.into(),
            webhook_secret,
        };
        let payload = serde_json::to_string(&created).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: created.provider.id,
            kind: "git_provider".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Application, CanCreate)]
    async fn create_gitlab_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: CreateGitlabProviderDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let (provider, webhook_secret) = self
            .service
            .create(CreateProvider {
                name: dto.provider.name,
                shared: dto.provider.shared.unwrap_or(true),
                credentials: ProviderCredentials::Gitlab {
                    url: dto.url,
                    internal_url: dto.internal_url,
                    application_id: dto.application_id,
                    redirect_uri: dto.redirect_uri,
                    secret: dto.secret,
                    access_token: dto.access_token,
                    refresh_token: dto.refresh_token,
                    group_name: dto.group_name,
                },
            })
            .await
            .map_err(map_db_error)?;
        let created = CreatedGitProviderResponseDto {
            provider: provider.into(),
            webhook_secret,
        };
        let payload = serde_json::to_string(&created).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: created.provider.id,
            kind: "git_provider".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Application, CanCreate)]
    async fn create_gitea_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: CreateGiteaProviderDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let (provider, webhook_secret) = self
            .service
            .create(CreateProvider {
                name: dto.provider.name,
                shared: dto.provider.shared.unwrap_or(true),
                credentials: ProviderCredentials::Gitea {
                    url: dto.url,
                    internal_url: dto.internal_url,
                    redirect_uri: dto.redirect_uri,
                    client_id: dto.client_id,
                    client_secret: dto.client_secret,
                    access_token: dto.access_token,
                    refresh_token: dto.refresh_token,
                    scopes: dto.scopes,
                },
            })
            .await
            .map_err(map_db_error)?;
        let created = CreatedGitProviderResponseDto {
            provider: provider.into(),
            webhook_secret,
        };
        let payload = serde_json::to_string(&created).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: created.provider.id,
            kind: "git_provider".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Application, CanCreate)]
    async fn create_bitbucket_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: CreateBitbucketProviderDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let (provider, webhook_secret) = self
            .service
            .create(CreateProvider {
                name: dto.provider.name,
                shared: dto.provider.shared.unwrap_or(true),
                credentials: ProviderCredentials::Bitbucket {
                    username: dto.username,
                    email: dto.email,
                    app_password: dto.app_password,
                    api_token: dto.api_token,
                    workspace: dto.workspace,
                },
            })
            .await
            .map_err(map_db_error)?;
        let created = CreatedGitProviderResponseDto {
            provider: provider.into(),
            webhook_secret,
        };
        let payload = serde_json::to_string(&created).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: created.provider.id,
            kind: "git_provider".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Application, CanCreate)]
    async fn update_github_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: CreateGithubProviderDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let provider = self
            .service
            .update(
                request.id,
                UpdateProvider {
                    name: dto.provider.name,
                    shared: dto.provider.shared,
                    credentials: ProviderCredentials::Github {
                        app_name: dto.app_name,
                        app_id: dto.app_id,
                        client_id: dto.client_id,
                        client_secret: dto.client_secret,
                        installation_id: dto.installation_id,
                        private_key: dto.private_key,
                    },
                },
            )
            .await
            .map_err(map_db_error)?;
        Response::ok(provider_to_resource(GitProviderResponseDto::from(provider)))
    }

    #[permission(Application, CanCreate)]
    async fn update_gitlab_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: CreateGitlabProviderDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let provider = self
            .service
            .update(
                request.id,
                UpdateProvider {
                    name: dto.provider.name,
                    shared: dto.provider.shared,
                    credentials: ProviderCredentials::Gitlab {
                        url: dto.url,
                        internal_url: dto.internal_url,
                        application_id: dto.application_id,
                        redirect_uri: dto.redirect_uri,
                        secret: dto.secret,
                        access_token: dto.access_token,
                        refresh_token: dto.refresh_token,
                        group_name: dto.group_name,
                    },
                },
            )
            .await
            .map_err(map_db_error)?;
        Response::ok(provider_to_resource(GitProviderResponseDto::from(provider)))
    }

    #[permission(Application, CanCreate)]
    async fn update_gitea_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: CreateGiteaProviderDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let provider = self
            .service
            .update(
                request.id,
                UpdateProvider {
                    name: dto.provider.name,
                    shared: dto.provider.shared,
                    credentials: ProviderCredentials::Gitea {
                        url: dto.url,
                        internal_url: dto.internal_url,
                        redirect_uri: dto.redirect_uri,
                        client_id: dto.client_id,
                        client_secret: dto.client_secret,
                        access_token: dto.access_token,
                        refresh_token: dto.refresh_token,
                        scopes: dto.scopes,
                    },
                },
            )
            .await
            .map_err(map_db_error)?;
        Response::ok(provider_to_resource(GitProviderResponseDto::from(provider)))
    }

    #[permission(Application, CanCreate)]
    async fn update_bitbucket_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: CreateBitbucketProviderDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let provider = self
            .service
            .update(
                request.id,
                UpdateProvider {
                    name: dto.provider.name,
                    shared: dto.provider.shared,
                    credentials: ProviderCredentials::Bitbucket {
                        username: dto.username,
                        email: dto.email,
                        app_password: dto.app_password,
                        api_token: dto.api_token,
                        workspace: dto.workspace,
                    },
                },
            )
            .await
            .map_err(map_db_error)?;
        Response::ok(provider_to_resource(GitProviderResponseDto::from(provider)))
    }

    #[permission(Application, CanDelete)]
    async fn delete_git_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service.delete(request.id).await.map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Application, CanRead)]
    async fn test_git_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        self.service
            .test_connection(request.id)
            .await
            .map_err(provider_error)?;
        Response::ok(OperationResult {
            success: true,
            operation: "test_connection".into(),
            ..Default::default()
        })
    }

    #[permission(Application, CanCreate)]
    async fn rotate_webhook_secret<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let webhook_secret = self
            .service
            .rotate_secret(request.id)
            .await
            .map_err(map_db_error)?;
        let resp = WebhookSecretResponseDto { webhook_secret };
        let payload = serde_json::to_string(&resp).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: request.id,
            kind: "webhook_secret".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Application, CanRead)]
    async fn list_repositories<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let repos = self
            .service
            .repositories(request.id)
            .await
            .map_err(provider_error)?;
        let items = repos
            .into_iter()
            .map(|r| {
                let payload = serde_json::to_string(&r).unwrap_or_default();
                ResourceResponse {
                    id: 0,
                    kind: "repository".into(),
                    payload,
                    ..Default::default()
                }
            })
            .collect();
        Response::ok(ResourceListResponse {
            items,
            ..Default::default()
        })
    }

    #[permission(Application, CanRead)]
    async fn list_branches<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let query: RepositoryReferenceQueryDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let refs = self
            .service
            .branches(request.id, &query.owner, &query.repository)
            .await
            .map_err(provider_error)?;
        let items = refs
            .into_iter()
            .map(|r| {
                let payload = serde_json::to_string(&r).unwrap_or_default();
                ResourceResponse {
                    id: 0,
                    kind: "git_reference".into(),
                    payload,
                    ..Default::default()
                }
            })
            .collect();
        Response::ok(ResourceListResponse {
            items,
            ..Default::default()
        })
    }

    #[permission(Application, CanRead)]
    async fn list_tags<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let query: RepositoryReferenceQueryDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let refs = self
            .service
            .tags(request.id, &query.owner, &query.repository)
            .await
            .map_err(provider_error)?;
        let items = refs
            .into_iter()
            .map(|r| {
                let payload = serde_json::to_string(&r).unwrap_or_default();
                ResourceResponse {
                    id: 0,
                    kind: "git_reference".into(),
                    payload,
                    ..Default::default()
                }
            })
            .collect();
        Response::ok(ResourceListResponse {
            items,
            ..Default::default()
        })
    }

    #[permission(Application, CanCreate)]
    async fn authorize<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let info = self
            .service
            .authorization(request.id)
            .await
            .map_err(provider_error)?;
        let payload = serde_json::to_string(&info).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: request.id,
            kind: "authorization".into(),
            payload,
            ..Default::default()
        })
    }

    async fn o_auth_callback<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: OAuthCallbackDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let provider = self
            .service
            .complete_authorization(
                request.id,
                &dto.state,
                dto.code.as_deref(),
                dto.installation_id.as_deref(),
            )
            .await
            .map_err(provider_error)?;
        Response::ok(provider_to_resource(GitProviderResponseDto::from(provider)))
    }

    #[permission(Application, CanDelete)]
    async fn disconnect<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service
            .disconnect(request.id)
            .await
            .map_err(provider_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Application, CanRead)]
    async fn get_collaborator_permission<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let query: CollaboratorPermissionQueryDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let perm = self
            .service
            .collaborator_permission(
                request.id,
                &query.owner,
                &query.repository,
                &query.username,
            )
            .await
            .map_err(provider_error)?;
        let payload = serde_json::to_string(&perm).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "get_collaborator_permission".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Application, CanRead)]
    async fn get_webhook_status<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let query: WebhookRepositoryDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let info = self
            .service
            .webhook_status(
                request.id,
                &query.owner,
                &query.repository,
                &query.callback_url,
            )
            .await
            .map_err(provider_error)?;
        let payload = serde_json::to_string(&info).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "get_webhook_status".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Application, CanCreate)]
    async fn install_webhook<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let dto: WebhookRepositoryDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let info = self
            .service
            .install_webhook(request.id, &dto.owner, &dto.repository, &dto.callback_url)
            .await
            .map_err(provider_error)?;
        let payload = serde_json::to_string(&info).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "install_webhook".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Application, CanCreate)]
    async fn recreate_webhook<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        let dto: WebhookRepositoryDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let info = self
            .service
            .recreate_webhook(request.id, &dto.owner, &dto.repository, &dto.callback_url)
            .await
            .map_err(provider_error)?;
        let payload = serde_json::to_string(&info).unwrap_or_default();
        Response::ok(OperationResult {
            success: true,
            operation: "recreate_webhook".into(),
            payload: Some(payload),
            ..Default::default()
        })
    }

    #[permission(Application, CanDelete)]
    async fn delete_webhook<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let query: WebhookRepositoryDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        self.service
            .remove_webhook(
                request.id,
                &query.owner,
                &query.repository,
                &query.callback_url,
            )
            .await
            .map_err(provider_error)?;
        Response::ok(Empty::default())
    }
}

// ── Helper DTOs for deserializing ResourceRequest payloads ────────────

use serde::{Deserialize, Serialize};

use crate::services::git_provider::{GitProviderConfigView, GitProviderView};

#[derive(Debug, Deserialize)]
struct ProviderBaseDto {
    name: String,
    shared: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct CreateGithubProviderDto {
    provider: ProviderBaseDto,
    app_name: Option<String>,
    app_id: Option<i64>,
    client_id: Option<String>,
    client_secret: Option<String>,
    installation_id: Option<String>,
    private_key: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CreateGitlabProviderDto {
    provider: ProviderBaseDto,
    url: String,
    internal_url: Option<String>,
    application_id: Option<String>,
    redirect_uri: Option<String>,
    secret: Option<String>,
    access_token: Option<String>,
    refresh_token: Option<String>,
    group_name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CreateGiteaProviderDto {
    provider: ProviderBaseDto,
    url: String,
    internal_url: Option<String>,
    redirect_uri: Option<String>,
    client_id: Option<String>,
    client_secret: Option<String>,
    access_token: Option<String>,
    refresh_token: Option<String>,
    scopes: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CreateBitbucketProviderDto {
    provider: ProviderBaseDto,
    username: Option<String>,
    email: Option<String>,
    app_password: Option<String>,
    api_token: Option<String>,
    workspace: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct GitProviderResponseDto {
    id: i64,
    name: String,
    provider_type: String,
    shared: bool,
    configured: bool,
    webhook_configured: bool,
    created_at: i64,
    updated_at: i64,
    config: GitProviderConfigDto,
}

#[derive(Debug, Clone, Serialize)]
struct GitProviderConfigDto {
    url: Option<String>,
    internal_url: Option<String>,
    app_name: Option<String>,
    app_id: Option<i64>,
    client_id: Option<String>,
    installation_id: Option<String>,
    application_id: Option<String>,
    redirect_uri: Option<String>,
    group_name: Option<String>,
    scopes: Option<String>,
    username: Option<String>,
    email: Option<String>,
    workspace: Option<String>,
    has_client_secret: bool,
    has_private_key: bool,
    has_access_token: bool,
    has_app_password: bool,
    has_api_token: bool,
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
struct CreatedGitProviderResponseDto {
    provider: GitProviderResponseDto,
    webhook_secret: String,
}

#[derive(Debug, Serialize)]
struct WebhookSecretResponseDto {
    webhook_secret: String,
}

#[derive(Debug, Deserialize)]
struct RepositoryReferenceQueryDto {
    owner: String,
    repository: String,
}

#[derive(Debug, Deserialize)]
struct WebhookRepositoryDto {
    owner: String,
    repository: String,
    callback_url: String,
}

#[derive(Debug, Deserialize)]
struct OAuthCallbackDto {
    code: Option<String>,
    state: String,
    installation_id: Option<String>,
    setup_action: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CollaboratorPermissionQueryDto {
    owner: String,
    repository: String,
    username: String,
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
