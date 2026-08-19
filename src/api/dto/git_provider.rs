use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::services::git_provider::GitProviderView;

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct ProviderBaseDto {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub shared: Option<bool>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct CreateGithubProviderDto {
    #[validate(nested)]
    pub provider: ProviderBaseDto,
    pub app_name: Option<String>,
    pub app_id: Option<i64>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub installation_id: Option<String>,
    pub private_key: Option<String>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct CreateGitlabProviderDto {
    #[validate(nested)]
    pub provider: ProviderBaseDto,
    #[validate(url)]
    pub url: String,
    pub internal_url: Option<String>,
    pub application_id: Option<String>,
    pub redirect_uri: Option<String>,
    pub secret: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub group_name: Option<String>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct CreateGiteaProviderDto {
    #[validate(nested)]
    pub provider: ProviderBaseDto,
    #[validate(url)]
    pub url: String,
    pub internal_url: Option<String>,
    pub redirect_uri: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub scopes: Option<String>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct CreateBitbucketProviderDto {
    #[validate(nested)]
    pub provider: ProviderBaseDto,
    pub username: Option<String>,
    pub email: Option<String>,
    pub app_password: Option<String>,
    pub api_token: Option<String>,
    pub workspace: Option<String>,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object, ts_rs::TS)]
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

#[derive(Debug, Clone, Serialize, poem_openapi::Object, ts_rs::TS)]
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

impl From<crate::services::git_provider::GitProviderConfigView> for GitProviderConfigDto {
    fn from(v: crate::services::git_provider::GitProviderConfigView) -> Self {
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

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct CreatedGitProviderResponseDto {
    pub provider: GitProviderResponseDto,
    pub webhook_secret: String,
}

#[derive(Debug, Serialize, poem_openapi::Object)]
pub struct WebhookSecretResponseDto {
    pub webhook_secret: String,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct RepositoryReferenceQueryDto {
    pub owner: String,
    pub repository: String,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct WebhookRepositoryDto {
    pub owner: String,
    pub repository: String,
    #[validate(url)]
    pub callback_url: String,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct OAuthCallbackDto {
    pub code: Option<String>,
    pub state: String,
    pub installation_id: Option<String>,
    pub setup_action: Option<String>,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct GithubManifestCallbackDto {
    pub code: String,
    pub installation_id: Option<String>,
    pub return_url: Option<String>,
}

#[derive(Debug, Deserialize, poem_openapi::Object)]
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
