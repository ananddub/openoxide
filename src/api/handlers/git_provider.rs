use auto_route::controller;
use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
};
use std::sync::Arc;

use crate::{
    api::dto::git_provider::{
        CollaboratorPermissionQueryDto, CreateBitbucketProviderDto, CreateGiteaProviderDto,
        CreateGithubProviderDto, CreateGitlabProviderDto, CreatedGitProviderResponseDto,
        GitProviderResponseDto, OAuthCallbackDto, RepositoryReferenceQueryDto,
        WebhookRepositoryDto, WebhookSecretResponseDto,
    },
    core::middleware::{
        permission::{
            AppCreatePermission, AppDeletePermission, AppReadPermission, RequirePermission,
        },
        validator::ValidatedJson,
    },
    services::git_provider::{
        CreateProvider, GitProviderService, ProviderCredentials, UpdateProvider,
    },
};

type ApiError = (StatusCode, String);

pub struct GitProviderController {
    service: Arc<GitProviderService>,
}

#[controller("/git-providers")]
impl GitProviderController {
    fn new(service: Arc<GitProviderService>) -> Self {
        Self { service }
    }

    #[get]
    async fn list(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppReadPermission>,
    ) -> Result<Json<Vec<GitProviderResponseDto>>, ApiError> {
        self.service
            .list()
            .await
            .map(|items| Json(items.into_iter().map(Into::into).collect()))
            .map_err(map_error)
    }

    #[get("/{id}")]
    async fn get(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppReadPermission>,
        Path(id): Path<i64>,
    ) -> Result<Json<GitProviderResponseDto>, ApiError> {
        self.service
            .get(id)
            .await
            .map(Into::into)
            .map(Json)
            .map_err(map_error)
    }

    #[post("/github")]
    async fn create_github(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        ValidatedJson(body): ValidatedJson<CreateGithubProviderDto>,
    ) -> Result<(StatusCode, Json<CreatedGitProviderResponseDto>), ApiError> {
        self.create(CreateProvider {
            name: body.provider.name,
            shared: body.provider.shared.unwrap_or(true),
            credentials: ProviderCredentials::Github {
                app_name: body.app_name,
                app_id: body.app_id,
                client_id: body.client_id,
                client_secret: body.client_secret,
                installation_id: body.installation_id,
                private_key: body.private_key,
            },
        })
        .await
    }

    #[post("/gitlab")]
    async fn create_gitlab(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        ValidatedJson(body): ValidatedJson<CreateGitlabProviderDto>,
    ) -> Result<(StatusCode, Json<CreatedGitProviderResponseDto>), ApiError> {
        self.create(CreateProvider {
            name: body.provider.name,
            shared: body.provider.shared.unwrap_or(true),
            credentials: ProviderCredentials::Gitlab {
                url: body.url,
                internal_url: body.internal_url,
                application_id: body.application_id,
                redirect_uri: body.redirect_uri,
                secret: body.secret,
                access_token: body.access_token,
                refresh_token: body.refresh_token,
                group_name: body.group_name,
            },
        })
        .await
    }

    #[post("/gitea")]
    async fn create_gitea(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        ValidatedJson(body): ValidatedJson<CreateGiteaProviderDto>,
    ) -> Result<(StatusCode, Json<CreatedGitProviderResponseDto>), ApiError> {
        self.create(CreateProvider {
            name: body.provider.name,
            shared: body.provider.shared.unwrap_or(true),
            credentials: ProviderCredentials::Gitea {
                url: body.url,
                internal_url: body.internal_url,
                redirect_uri: body.redirect_uri,
                client_id: body.client_id,
                client_secret: body.client_secret,
                access_token: body.access_token,
                refresh_token: body.refresh_token,
                scopes: body.scopes,
            },
        })
        .await
    }

    #[post("/bitbucket")]
    async fn create_bitbucket(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        ValidatedJson(body): ValidatedJson<CreateBitbucketProviderDto>,
    ) -> Result<(StatusCode, Json<CreatedGitProviderResponseDto>), ApiError> {
        self.create(CreateProvider {
            name: body.provider.name,
            shared: body.provider.shared.unwrap_or(true),
            credentials: ProviderCredentials::Bitbucket {
                username: body.username,
                email: body.email,
                app_password: body.app_password,
                api_token: body.api_token,
                workspace: body.workspace,
            },
        })
        .await
    }

    #[put("/{id}/github")]
    async fn update_github(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<CreateGithubProviderDto>,
    ) -> Result<Json<GitProviderResponseDto>, ApiError> {
        self.update(
            id,
            UpdateProvider {
                name: body.provider.name,
                shared: body.provider.shared,
                credentials: ProviderCredentials::Github {
                    app_name: body.app_name,
                    app_id: body.app_id,
                    client_id: body.client_id,
                    client_secret: body.client_secret,
                    installation_id: body.installation_id,
                    private_key: body.private_key,
                },
            },
        )
        .await
    }

    #[put("/{id}/gitlab")]
    async fn update_gitlab(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<CreateGitlabProviderDto>,
    ) -> Result<Json<GitProviderResponseDto>, ApiError> {
        self.update(
            id,
            UpdateProvider {
                name: body.provider.name,
                shared: body.provider.shared,
                credentials: ProviderCredentials::Gitlab {
                    url: body.url,
                    internal_url: body.internal_url,
                    application_id: body.application_id,
                    redirect_uri: body.redirect_uri,
                    secret: body.secret,
                    access_token: body.access_token,
                    refresh_token: body.refresh_token,
                    group_name: body.group_name,
                },
            },
        )
        .await
    }

    #[put("/{id}/gitea")]
    async fn update_gitea(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<CreateGiteaProviderDto>,
    ) -> Result<Json<GitProviderResponseDto>, ApiError> {
        self.update(
            id,
            UpdateProvider {
                name: body.provider.name,
                shared: body.provider.shared,
                credentials: ProviderCredentials::Gitea {
                    url: body.url,
                    internal_url: body.internal_url,
                    redirect_uri: body.redirect_uri,
                    client_id: body.client_id,
                    client_secret: body.client_secret,
                    access_token: body.access_token,
                    refresh_token: body.refresh_token,
                    scopes: body.scopes,
                },
            },
        )
        .await
    }

    #[put("/{id}/bitbucket")]
    async fn update_bitbucket(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<CreateBitbucketProviderDto>,
    ) -> Result<Json<GitProviderResponseDto>, ApiError> {
        self.update(
            id,
            UpdateProvider {
                name: body.provider.name,
                shared: body.provider.shared,
                credentials: ProviderCredentials::Bitbucket {
                    username: body.username,
                    email: body.email,
                    app_password: body.app_password,
                    api_token: body.api_token,
                    workspace: body.workspace,
                },
            },
        )
        .await
    }

    #[post("/{id}/webhook-secret/rotate")]
    async fn rotate_secret(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
    ) -> Result<Json<WebhookSecretResponseDto>, ApiError> {
        self.service
            .rotate_secret(id)
            .await
            .map(|webhook_secret| Json(WebhookSecretResponseDto { webhook_secret }))
            .map_err(map_error)
    }

    #[post("/{id}/test")]
    async fn test_connection(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppReadPermission>,
        Path(id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .test_connection(id)
            .await
            .map(|_| StatusCode::OK)
            .map_err(provider_error)
    }

    #[get("/{id}/repositories")]
    async fn repositories(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppReadPermission>,
        Path(id): Path<i64>,
    ) -> Result<Json<Vec<crate::utils::provider::discovery::RepositoryInfo>>, ApiError> {
        self.service
            .repositories(id)
            .await
            .map(Json)
            .map_err(provider_error)
    }

    #[get("/{id}/branches")]
    async fn branches(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppReadPermission>,
        Path(id): Path<i64>,
        Query(query): Query<RepositoryReferenceQueryDto>,
    ) -> Result<Json<Vec<crate::utils::provider::discovery::GitReferenceInfo>>, ApiError> {
        self.service
            .branches(id, &query.owner, &query.repository)
            .await
            .map(Json)
            .map_err(provider_error)
    }

    #[get("/{id}/tags")]
    async fn tags(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppReadPermission>,
        Path(id): Path<i64>,
        Query(query): Query<RepositoryReferenceQueryDto>,
    ) -> Result<Json<Vec<crate::utils::provider::discovery::GitReferenceInfo>>, ApiError> {
        self.service
            .tags(id, &query.owner, &query.repository)
            .await
            .map(Json)
            .map_err(provider_error)
    }

    #[get("/{id}/authorize")]
    async fn authorize(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
    ) -> Result<Json<crate::utils::provider::oauth::AuthorizationInfo>, ApiError> {
        self.service
            .authorization(id)
            .await
            .map(Json)
            .map_err(provider_error)
    }

    #[get("/{id}/oauth/callback")]
    async fn oauth_callback(
        &self,
        Path(id): Path<i64>,
        Query(query): Query<OAuthCallbackDto>,
    ) -> Result<Json<GitProviderResponseDto>, ApiError> {
        self.service
            .complete_authorization(
                id,
                &query.state,
                query.code.as_deref(),
                query.installation_id.as_deref(),
            )
            .await
            .map(Into::into)
            .map(Json)
            .map_err(provider_error)
    }

    #[post("/{id}/disconnect")]
    async fn disconnect(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeletePermission>,
        Path(id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .disconnect(id)
            .await
            .map(|_| StatusCode::NO_CONTENT)
            .map_err(provider_error)
    }

    #[get("/{id}/collaborator-permission")]
    async fn collaborator_permission(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppReadPermission>,
        Path(id): Path<i64>,
        Query(query): Query<CollaboratorPermissionQueryDto>,
    ) -> Result<Json<crate::utils::provider::discovery::CollaboratorPermission>, ApiError> {
        self.service
            .collaborator_permission(id, &query.owner, &query.repository, &query.username)
            .await
            .map(Json)
            .map_err(provider_error)
    }

    #[get("/{id}/webhook/status")]
    async fn webhook_status(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppReadPermission>,
        Path(id): Path<i64>,
        Query(query): Query<WebhookRepositoryDto>,
    ) -> Result<Json<Option<crate::utils::provider::discovery::ProviderWebhookInfo>>, ApiError>
    {
        self.service
            .webhook_status(id, &query.owner, &query.repository, &query.callback_url)
            .await
            .map(Json)
            .map_err(provider_error)
    }

    #[post("/{id}/webhook/install")]
    async fn install_webhook(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<WebhookRepositoryDto>,
    ) -> Result<Json<crate::utils::provider::discovery::ProviderWebhookInfo>, ApiError> {
        self.service
            .install_webhook(id, &body.owner, &body.repository, &body.callback_url)
            .await
            .map(Json)
            .map_err(provider_error)
    }

    #[post("/{id}/webhook/recreate")]
    async fn recreate_webhook(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppCreatePermission>,
        Path(id): Path<i64>,
        ValidatedJson(body): ValidatedJson<WebhookRepositoryDto>,
    ) -> Result<Json<crate::utils::provider::discovery::ProviderWebhookInfo>, ApiError> {
        self.service
            .recreate_webhook(id, &body.owner, &body.repository, &body.callback_url)
            .await
            .map(Json)
            .map_err(provider_error)
    }

    #[delete("/{id}/webhook")]
    async fn remove_webhook(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeletePermission>,
        Path(id): Path<i64>,
        Query(query): Query<WebhookRepositoryDto>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .remove_webhook(id, &query.owner, &query.repository, &query.callback_url)
            .await
            .map(|_| StatusCode::NO_CONTENT)
            .map_err(provider_error)
    }

    #[delete("/{id}")]
    async fn delete(
        &self,
        RequirePermission(_claims, _): RequirePermission<AppDeletePermission>,
        Path(id): Path<i64>,
    ) -> Result<StatusCode, ApiError> {
        self.service
            .delete(id)
            .await
            .map(|_| StatusCode::NO_CONTENT)
            .map_err(map_error)
    }
}

impl GitProviderController {
    async fn create(
        &self,
        input: CreateProvider,
    ) -> Result<(StatusCode, Json<CreatedGitProviderResponseDto>), ApiError> {
        self.service
            .create(input)
            .await
            .map(|(provider, webhook_secret)| {
                (
                    StatusCode::CREATED,
                    Json(CreatedGitProviderResponseDto {
                        provider: provider.into(),
                        webhook_secret,
                    }),
                )
            })
            .map_err(map_error)
    }

    async fn update(
        &self,
        id: i64,
        input: UpdateProvider,
    ) -> Result<Json<GitProviderResponseDto>, ApiError> {
        self.service
            .update(id, input)
            .await
            .map(Into::into)
            .map(Json)
            .map_err(map_error)
    }
}

fn map_error(error: sqlx::Error) -> ApiError {
    match error {
        sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "Git provider not found".into()),
        sqlx::Error::Protocol(message) if message.contains("assigned") => {
            (StatusCode::CONFLICT, message)
        }
        sqlx::Error::Database(ref error) if error.is_unique_violation() => {
            (StatusCode::CONFLICT, error.message().into())
        }
        other => {
            tracing::error!(error = %other, "Git provider operation failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Git provider operation failed".into(),
            )
        }
    }
}

fn provider_error(error: String) -> ApiError {
    let normalized = error.to_ascii_lowercase();
    let status = if normalized.contains("not found") {
        StatusCode::NOT_FOUND
    } else if normalized.contains("required")
        || normalized.contains("invalid")
        || normalized.contains("unsupported")
        || normalized.contains("must end with")
        || normalized.contains("does not support")
    {
        StatusCode::BAD_REQUEST
    } else {
        StatusCode::BAD_GATEWAY
    };
    (status, error)
}
