use connectrpc::{RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    CreateBitbucketProviderRequest, CreateBitbucketProviderResponse, CreateGiteaProviderRequest,
    CreateGiteaProviderResponse, CreateGithubProviderRequest, CreateGithubProviderResponse,
    CreateGitlabProviderRequest, CreateGitlabProviderResponse, DeleteGitProviderRequest, Empty,
    GetGitProviderRequest, GetGitProviderResponse, GitAuthorizeRequest, GitAuthorizeResponse,
    GitDeleteWebhookRequest, GitDisconnectRequest, GitGetCollaboratorPermissionRequest,
    GitGetCollaboratorPermissionResponse, GitGetWebhookStatusRequest, GitGetWebhookStatusResponse,
    GitInstallWebhookRequest, GitInstallWebhookResponse, GitListRepositoriesRequest,
    GitListRepositoriesResponse, GitListRepositoryBranchesRequest,
    GitListRepositoryBranchesResponse, GitListTagsRequest, GitListTagsResponse,
    GitOAuthCallbackRequest, GitOAuthCallbackResponse, GitProviderService, GitProviderServiceExt,
    GitRecreateWebhookRequest, GitRecreateWebhookResponse, GitRotateWebhookSecretRequest,
    GitRotateWebhookSecretResponse, ListGitProvidersRequest, ListGitProvidersResponse,
    TestGitProviderRequest, TestGitProviderResponse, UpdateBitbucketProviderRequest,
    UpdateBitbucketProviderResponse, UpdateGiteaProviderRequest, UpdateGiteaProviderResponse,
    UpdateGithubProviderRequest, UpdateGithubProviderResponse, UpdateGitlabProviderRequest,
    UpdateGitlabProviderResponse,
};
use crate::services::integrations::git::{CreateProvider, ProviderCredentials, UpdateProvider};

use super::helper::*;
use super::init::GitProviderController;

#[auto_grpc::service(connect = true, auth = true)]
impl GitProviderService for GitProviderController {
    #[permission(Application, CanRead)]
    async fn list_git_providers<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, ListGitProvidersRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListGitProvidersResponse> + Send + use<'a>> {
        let org_id = auth.organization_id;
        let items = self.service.list(org_id).await.map_err(map_db_error)?;
        let providers = items.into_iter().map(provider_to_proto).collect();
        Response::ok(ListGitProvidersResponse { providers })
    }

    #[permission(Application, CanRead)]
    async fn get_git_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GetGitProviderRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GetGitProviderResponse> + Send + use<'a>> {
        let org_id = auth.organization_id;
        let provider = self
            .service
            .get(request.id, org_id)
            .await
            .map_err(map_db_error)?;
        Response::ok(GetGitProviderResponse {
            provider: ::buffa::MessageField::some(provider_to_proto(provider)),
        })
    }

    #[permission(Application, CanCreate)]
    async fn create_github_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, CreateGithubProviderRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<CreateGithubProviderResponse> + Send + use<'a>>
    {
        let org_id = auth.organization_id;
        let (provider, webhook_secret) = self
            .service
            .create(
                CreateProvider {
                    name: request.name.to_owned(),
                    shared: request.shared.unwrap_or(true),
                    credentials: ProviderCredentials::Github {
                        app_name: request.app_name.map(str::to_owned),
                        app_id: request.app_id,
                        client_id: request.client_id.map(str::to_owned),
                        client_secret: request.client_secret.map(str::to_owned),
                        installation_id: request.installation_id.map(str::to_owned),
                        private_key: request.private_key.map(str::to_owned),
                    },
                },
                org_id,
            )
            .await
            .map_err(map_db_error)?;
        Response::ok(CreateGithubProviderResponse {
            provider: ::buffa::MessageField::some(provider_to_proto(provider)),
            webhook_secret,
        })
    }

    #[permission(Application, CanCreate)]
    async fn create_gitlab_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, CreateGitlabProviderRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<CreateGitlabProviderResponse> + Send + use<'a>>
    {
        let org_id = auth.organization_id;
        let (provider, webhook_secret) = self
            .service
            .create(
                CreateProvider {
                    name: request.name.to_owned(),
                    shared: request.shared.unwrap_or(true),
                    credentials: ProviderCredentials::Gitlab {
                        url: request.url.to_owned(),
                        internal_url: request.internal_url.map(str::to_owned),
                        application_id: request.application_id.map(str::to_owned),
                        redirect_uri: request.redirect_uri.map(str::to_owned),
                        secret: request.secret.map(str::to_owned),
                        access_token: request.access_token.map(str::to_owned),
                        refresh_token: request.refresh_token.map(str::to_owned),
                        group_name: request.group_name.map(str::to_owned),
                    },
                },
                org_id,
            )
            .await
            .map_err(map_db_error)?;
        Response::ok(CreateGitlabProviderResponse {
            provider: ::buffa::MessageField::some(provider_to_proto(provider)),
            webhook_secret,
        })
    }

    #[permission(Application, CanCreate)]
    async fn create_gitea_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, CreateGiteaProviderRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<CreateGiteaProviderResponse> + Send + use<'a>>
    {
        let org_id = auth.organization_id;
        let (provider, webhook_secret) = self
            .service
            .create(
                CreateProvider {
                    name: request.name.to_owned(),
                    shared: request.shared.unwrap_or(true),
                    credentials: ProviderCredentials::Gitea {
                        url: request.url.to_owned(),
                        internal_url: request.internal_url.map(str::to_owned),
                        redirect_uri: request.redirect_uri.map(str::to_owned),
                        client_id: request.client_id.map(str::to_owned),
                        client_secret: request.client_secret.map(str::to_owned),
                        access_token: request.access_token.map(str::to_owned),
                        refresh_token: request.refresh_token.map(str::to_owned),
                        scopes: request.scopes.map(str::to_owned),
                    },
                },
                org_id,
            )
            .await
            .map_err(map_db_error)?;
        Response::ok(CreateGiteaProviderResponse {
            provider: ::buffa::MessageField::some(provider_to_proto(provider)),
            webhook_secret,
        })
    }

    #[permission(Application, CanCreate)]
    async fn create_bitbucket_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, CreateBitbucketProviderRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<CreateBitbucketProviderResponse> + Send + use<'a>>
    {
        let org_id = auth.organization_id;
        let (provider, webhook_secret) = self
            .service
            .create(
                CreateProvider {
                    name: request.name.to_owned(),
                    shared: request.shared.unwrap_or(true),
                    credentials: ProviderCredentials::Bitbucket {
                        username: request.username.map(str::to_owned),
                        email: request.email.map(str::to_owned),
                        app_password: request.app_password.map(str::to_owned),
                        api_token: request.api_token.map(str::to_owned),
                        workspace: request.workspace.map(str::to_owned),
                    },
                },
                org_id,
            )
            .await
            .map_err(map_db_error)?;
        Response::ok(CreateBitbucketProviderResponse {
            provider: ::buffa::MessageField::some(provider_to_proto(provider)),
            webhook_secret,
        })
    }

    #[permission(Application, CanCreate)]
    async fn update_github_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, UpdateGithubProviderRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<UpdateGithubProviderResponse> + Send + use<'a>>
    {
        let org_id = auth.organization_id;
        let provider = self
            .service
            .update(
                request.id,
                UpdateProvider {
                    name: request.name.to_owned(),
                    shared: request.shared,
                    credentials: ProviderCredentials::Github {
                        app_name: request.app_name.map(str::to_owned),
                        app_id: request.app_id,
                        client_id: request.client_id.map(str::to_owned),
                        client_secret: request.client_secret.map(str::to_owned),
                        installation_id: request.installation_id.map(str::to_owned),
                        private_key: request.private_key.map(str::to_owned),
                    },
                },
                org_id,
            )
            .await
            .map_err(map_db_error)?;
        Response::ok(UpdateGithubProviderResponse {
            provider: ::buffa::MessageField::some(provider_to_proto(provider)),
        })
    }

    #[permission(Application, CanCreate)]
    async fn update_gitlab_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, UpdateGitlabProviderRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<UpdateGitlabProviderResponse> + Send + use<'a>>
    {
        let org_id = auth.organization_id;
        let provider = self
            .service
            .update(
                request.id,
                UpdateProvider {
                    name: request.name.to_owned(),
                    shared: request.shared,
                    credentials: ProviderCredentials::Gitlab {
                        url: request.url.map(str::to_owned).unwrap_or_default(),
                        internal_url: request.internal_url.map(str::to_owned),
                        application_id: request.application_id.map(str::to_owned),
                        redirect_uri: request.redirect_uri.map(str::to_owned),
                        secret: request.secret.map(str::to_owned),
                        access_token: request.access_token.map(str::to_owned),
                        refresh_token: request.refresh_token.map(str::to_owned),
                        group_name: request.group_name.map(str::to_owned),
                    },
                },
                org_id,
            )
            .await
            .map_err(map_db_error)?;
        Response::ok(UpdateGitlabProviderResponse {
            provider: ::buffa::MessageField::some(provider_to_proto(provider)),
        })
    }

    #[permission(Application, CanCreate)]
    async fn update_gitea_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, UpdateGiteaProviderRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<UpdateGiteaProviderResponse> + Send + use<'a>>
    {
        let org_id = auth.organization_id;
        let provider = self
            .service
            .update(
                request.id,
                UpdateProvider {
                    name: request.name.to_owned(),
                    shared: request.shared,
                    credentials: ProviderCredentials::Gitea {
                        url: request.url.map(str::to_owned).unwrap_or_default(),
                        internal_url: request.internal_url.map(str::to_owned),
                        redirect_uri: request.redirect_uri.map(str::to_owned),
                        client_id: request.client_id.map(str::to_owned),
                        client_secret: request.client_secret.map(str::to_owned),
                        access_token: request.access_token.map(str::to_owned),
                        refresh_token: request.refresh_token.map(str::to_owned),
                        scopes: request.scopes.map(str::to_owned),
                    },
                },
                org_id,
            )
            .await
            .map_err(map_db_error)?;
        Response::ok(UpdateGiteaProviderResponse {
            provider: ::buffa::MessageField::some(provider_to_proto(provider)),
        })
    }

    #[permission(Application, CanCreate)]
    async fn update_bitbucket_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, UpdateBitbucketProviderRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<UpdateBitbucketProviderResponse> + Send + use<'a>>
    {
        let org_id = auth.organization_id;
        let provider = self
            .service
            .update(
                request.id,
                UpdateProvider {
                    name: request.name.to_owned(),
                    shared: request.shared,
                    credentials: ProviderCredentials::Bitbucket {
                        username: request.username.map(str::to_owned),
                        email: request.email.map(str::to_owned),
                        app_password: request.app_password.map(str::to_owned),
                        api_token: request.api_token.map(str::to_owned),
                        workspace: request.workspace.map(str::to_owned),
                    },
                },
                org_id,
            )
            .await
            .map_err(map_db_error)?;
        Response::ok(UpdateBitbucketProviderResponse {
            provider: ::buffa::MessageField::some(provider_to_proto(provider)),
        })
    }

    #[permission(Application, CanDelete)]
    async fn delete_git_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DeleteGitProviderRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let org_id = auth.organization_id;
        self.service
            .delete(request.id, org_id)
            .await
            .map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Application, CanRead)]
    async fn test_git_provider<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, TestGitProviderRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<TestGitProviderResponse> + Send + use<'a>> {
        let org_id = auth.organization_id;
        self.service
            .test_connection(request.id, org_id)
            .await
            .map_err(provider_error)?;
        Response::ok(TestGitProviderResponse {
            success: true,
            message: "connection test succeeded".into(),
        })
    }

    #[permission(Application, CanCreate)]
    async fn rotate_webhook_secret<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GitRotateWebhookSecretRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GitRotateWebhookSecretResponse> + Send + use<'a>>
    {
        let org_id = auth.organization_id;
        let webhook_secret = self
            .service
            .rotate_secret(request.id, org_id)
            .await
            .map_err(map_db_error)?;
        Response::ok(GitRotateWebhookSecretResponse { webhook_secret })
    }

    #[permission(Application, CanRead)]
    async fn list_repositories<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GitListRepositoriesRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GitListRepositoriesResponse> + Send + use<'a>>
    {
        let org_id = auth.organization_id;
        let repos = self
            .service
            .repositories(request.provider_id, org_id)
            .await
            .map_err(provider_error)?;
        let repositories = repos.into_iter().map(repository_to_proto).collect();
        Response::ok(GitListRepositoriesResponse { repositories })
    }

    #[permission(Application, CanRead)]
    async fn list_branches<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GitListRepositoryBranchesRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GitListRepositoryBranchesResponse> + Send + use<'a>>
    {
        let org_id = auth.organization_id;
        let refs = self
            .service
            .branches(
                request.provider_id,
                request.owner,
                request.repository,
                org_id,
            )
            .await
            .map_err(provider_error)?;
        let branches = refs.into_iter().map(reference_to_proto).collect();
        Response::ok(GitListRepositoryBranchesResponse { branches })
    }

    #[permission(Application, CanRead)]
    async fn list_tags<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GitListTagsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GitListTagsResponse> + Send + use<'a>> {
        let org_id = auth.organization_id;
        let refs = self
            .service
            .tags(
                request.provider_id,
                request.owner,
                request.repository,
                org_id,
            )
            .await
            .map_err(provider_error)?;
        let tags = refs.into_iter().map(reference_to_proto).collect();
        Response::ok(GitListTagsResponse { tags })
    }

    #[permission(Application, CanCreate)]
    async fn authorize<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GitAuthorizeRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GitAuthorizeResponse> + Send + use<'a>> {
        let org_id = auth.organization_id;
        let info = self
            .service
            .authorization(request.provider_id, org_id)
            .await
            .map_err(provider_error)?;
        Response::ok(GitAuthorizeResponse {
            url: info.url,
            state: info.state,
        })
    }

    async fn o_auth_callback<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GitOAuthCallbackRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GitOAuthCallbackResponse> + Send + use<'a>> {
        let org_id = auth.organization_id;
        let provider = self
            .service
            .complete_authorization(
                request.provider_id,
                request.state,
                request.code,
                request.installation_id,
                org_id,
            )
            .await
            .map_err(provider_error)?;
        Response::ok(GitOAuthCallbackResponse {
            provider: ::buffa::MessageField::some(provider_to_proto(provider)),
        })
    }

    #[permission(Application, CanDelete)]
    async fn disconnect<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GitDisconnectRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let org_id = auth.organization_id;
        self.service
            .disconnect(request.provider_id, org_id)
            .await
            .map_err(provider_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Application, CanRead)]
    async fn get_collaborator_permission<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GitGetCollaboratorPermissionRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<GitGetCollaboratorPermissionResponse> + Send + use<'a>,
    > {
        let org_id = auth.organization_id;
        let permission = self
            .service
            .collaborator_permission(
                request.provider_id,
                request.owner,
                request.repository,
                request.username,
                org_id,
            )
            .await
            .map_err(provider_error)?;
        Response::ok(GitGetCollaboratorPermissionResponse {
            permission: ::buffa::MessageField::some(permission_to_proto(permission)),
        })
    }

    #[permission(Application, CanRead)]
    async fn get_webhook_status<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GitGetWebhookStatusRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GitGetWebhookStatusResponse> + Send + use<'a>>
    {
        let org_id = auth.organization_id;
        let info = self
            .service
            .webhook_status(
                request.provider_id,
                request.owner,
                request.repository,
                request.callback_url,
                org_id,
            )
            .await
            .map_err(provider_error)?;
        let webhook = info
            .map(webhook_info_to_proto)
            .map(::buffa::MessageField::some)
            .unwrap_or_default();
        Response::ok(GitGetWebhookStatusResponse { webhook })
    }

    #[permission(Application, CanCreate)]
    async fn install_webhook<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GitInstallWebhookRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GitInstallWebhookResponse> + Send + use<'a>> {
        let org_id = auth.organization_id;
        let info = self
            .service
            .install_webhook(
                request.provider_id,
                request.owner,
                request.repository,
                request.callback_url,
                org_id,
            )
            .await
            .map_err(provider_error)?;
        Response::ok(GitInstallWebhookResponse {
            webhook: ::buffa::MessageField::some(webhook_info_to_proto(info)),
        })
    }

    #[permission(Application, CanCreate)]
    async fn recreate_webhook<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GitRecreateWebhookRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GitRecreateWebhookResponse> + Send + use<'a>>
    {
        let org_id = auth.organization_id;
        let info = self
            .service
            .recreate_webhook(
                request.provider_id,
                request.owner,
                request.repository,
                request.callback_url,
                org_id,
            )
            .await
            .map_err(provider_error)?;
        Response::ok(GitRecreateWebhookResponse {
            webhook: ::buffa::MessageField::some(webhook_info_to_proto(info)),
        })
    }

    #[permission(Application, CanDelete)]
    async fn delete_webhook<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GitDeleteWebhookRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let org_id = auth.organization_id;
        self.service
            .remove_webhook(
                request.provider_id,
                request.owner,
                request.repository,
                request.callback_url,
                org_id,
            )
            .await
            .map_err(provider_error)?;
        Response::ok(Empty::default())
    }
}
