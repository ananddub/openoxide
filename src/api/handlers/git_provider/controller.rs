use connectrpc::{ConnectError, RequestContext, Response, ServiceRequest, ServiceResult};
use futures::StreamExt;

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, GitProviderService, GitProviderServiceExt, OperationResult, ResourceListResponse,
    ResourceQuery, ResourceRequest, ResourceResponse,
};
use crate::db::reactive::realtime;
use crate::flash_gen::models::GitProviders;
use crate::services::integrations::git::{CreateProvider, ProviderCredentials, UpdateProvider};

use super::helper::*;
use super::init::GitProviderController;

#[auto_grpc::service(connect = true, auth = true)]
impl GitProviderService for GitProviderController {
    #[permission(Application, CanRead)]
    async fn stream_list_git_providers(
        &self,
        ctx: RequestContext,
        _query: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<ResourceListResponse> + Send + use<>,
        >,
    > {
        let stream = {
            let sql = format!(
                "SELECT * FROM git_providers WHERE organization_id = {}",
                auth.organization_id
            );
            realtime()
                .watch_query::<GitProviders>(sql)
                .await
                .map_err(map_db_error)?
        };
        Response::stream_ok(stream.map(move |rows| {
            Ok(ResourceListResponse {
                items: rows
                    .into_iter()
                    .map(git_provider_response_from_model)
                    .map(provider_to_resource)
                    .collect(),
                ..Default::default()
            })
        }))
    }

    #[permission(Application, CanRead)]
    async fn stream_git_provider(
        &self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<ResourceResponse> + Send + use<>,
        >,
    > {
        let id = request.id;
        let stream = {
            let sql = format!(
                "SELECT * FROM git_providers WHERE id = {id} AND organization_id = {}",
                auth.organization_id
            );
            realtime()
                .watch_query::<GitProviders>(sql)
                .await
                .map_err(map_db_error)?
        };
        Response::stream_ok(stream.map(move |mut rows| match rows.pop() {
            Some(item) => Ok(provider_to_resource(git_provider_response_from_model(item))),
            None => Err(ConnectError::not_found("git provider not found")),
        }))
    }

    #[permission(Application, CanRead)]
    async fn list_git_providers<'a>(
        &'a self,
        ctx: RequestContext,
        _query: ServiceRequest<'_, ResourceQuery>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceListResponse> + Send + use<'a>> {
        let items = self
            .service
            .list(auth.organization_id)
            .await
            .map_err(map_db_error)?;
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
        let provider = self
            .service
            .get(request.id, auth.organization_id)
            .await
            .map_err(map_db_error)?;
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
            }, auth.organization_id)
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
            }, auth.organization_id)
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
            }, auth.organization_id)
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
            }, auth.organization_id)
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
                auth.organization_id,
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
                auth.organization_id,
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
                auth.organization_id,
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
                auth.organization_id,
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
        self.service
            .delete(request.id, auth.organization_id)
            .await
            .map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Application, CanRead)]
    async fn test_git_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<OperationResult> + Send + use<'a>> {
        self.service
            .test_connection(request.id, auth.organization_id)
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
            .rotate_secret(request.id, auth.organization_id)
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
            .repositories(request.id, auth.organization_id)
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
            .branches(request.id, &query.owner, &query.repository, auth.organization_id)
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
            .tags(request.id, &query.owner, &query.repository, auth.organization_id)
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
            .authorization(request.id, auth.organization_id)
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
        ctx: RequestContext,
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
                auth.organization_id,
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
            .disconnect(request.id, auth.organization_id)
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
                auth.organization_id,
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
                auth.organization_id,
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
            .install_webhook(
                request.id,
                &dto.owner,
                &dto.repository,
                &dto.callback_url,
                auth.organization_id,
            )
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
            .recreate_webhook(
                request.id,
                &dto.owner,
                &dto.repository,
                &dto.callback_url,
                auth.organization_id,
            )
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
                auth.organization_id,
            )
            .await
            .map_err(provider_error)?;
        Response::ok(Empty::default())
    }
}
