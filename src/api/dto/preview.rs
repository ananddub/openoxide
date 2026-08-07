use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, poem_openapi::Object)]
pub struct PreviewListQueryDto {
    pub active_only: Option<bool>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct CreatePreviewDeploymentDto {
    pub provider: String,
    #[validate(length(min = 1, max = 255))]
    pub owner: String,
    #[validate(length(min = 1, max = 255))]
    pub repository: String,
    #[validate(length(min = 1, max = 100))]
    pub pull_request_number: String,
    #[validate(length(min = 1, max = 255))]
    pub source_branch: String,
    pub source_owner: Option<String>,
    pub source_repository: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub target_branch: String,
    pub commit_sha: Option<String>,
    pub author: Option<String>,
}
