use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::db::models::{
    application_middlewares::ApplicationMiddleware, applications::Application, domains::Domain,
    mounts::Mount, patches::Patch, ports::Port, redirects::Redirect, security::Security,
};

#[derive(Debug, Serialize, Deserialize, poem_openapi::Object)]
pub struct ApplicationExportQueryDto {
    #[serde(default)]
    pub include_secrets: bool,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct ApplicationExportArchiveDto {
    pub format: String,
    pub schema_version: i64,
    pub archive: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationExportBundleDto {
    pub schema_version: u32,
    pub exported_at: i64,
    pub secrets_included: bool,
    pub application: Application,
    pub domains: Vec<Domain>,
    pub ports: Vec<Port>,
    pub mounts: Vec<Mount>,
    pub redirects: Vec<Redirect>,
    pub security: Vec<Security>,
    pub patches: Vec<Patch>,
    pub middlewares: Vec<ApplicationMiddleware>,
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct ImportApplicationDto {
    #[validate(range(min = 1))]
    pub target_environment_id: i64,
    pub target_server_id: Option<i64>,
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    #[validate(length(min = 2))]
    pub archive: String,
}
