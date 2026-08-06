#[derive(Debug, Clone)]
pub struct GlobalSearchOptions {
    pub query: String,
    pub limit: i64,
    pub offset: i64,
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, poem_openapi::Object)]
pub struct GlobalResourceDto {
    pub resource_type: String,
    pub id: String,
    pub name: String,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, poem_openapi::Enum)]
pub enum BulkDeploymentAction {
    Cancel,
}

#[derive(Debug, Clone, serde::Deserialize, poem_openapi::Object)]
pub struct BulkDeploymentRequest {
    pub action: BulkDeploymentAction,
    pub deployment_ids: Vec<i64>,
}

#[derive(Debug, Clone, serde::Serialize, poem_openapi::Object)]
pub struct BulkDeploymentResult {
    pub deployment_id: i64,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, poem_openapi::Enum)]
pub enum BulkResourceKind {
    Application,
    Compose,
    Database,
    Server,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, poem_openapi::Enum)]
pub enum BulkResourceAction {
    Start,
    Stop,
    Redeploy,
    Delete,
    Cleanup,
}

#[derive(Debug, Clone, serde::Deserialize, poem_openapi::Object)]
pub struct BulkResourceItem {
    pub id: i64,
    pub database_kind: Option<crate::services::database::DatabaseKind>,
}

#[derive(Debug, Clone, serde::Deserialize, poem_openapi::Object)]
pub struct BulkResourceRequest {
    pub resource_kind: BulkResourceKind,
    pub action: BulkResourceAction,
    pub items: Vec<BulkResourceItem>,
}

#[derive(Debug, Clone, serde::Serialize, poem_openapi::Object)]
pub struct BulkResourceResult {
    pub id: i64,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Clone, serde::Serialize, poem_openapi::Object)]
pub struct ServerDependencyView {
    pub server_id: i64,
    pub applications: i64,
    pub build_assignments: i64,
    pub compose_projects: i64,
    pub databases: i64,
    pub certificates: i64,
    pub schedules: i64,
    pub safe_to_delete: bool,
}
