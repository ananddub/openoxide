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

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct GitProviderResponseDto {
    pub id: i64,
    pub name: String,
    pub provider_type: String,
    pub shared: bool,
    pub configured: bool,
    pub webhook_configured: bool,
    pub created_at: i64,
    pub updated_at: i64,
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
        }
    }
}
