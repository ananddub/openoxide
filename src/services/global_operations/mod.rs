mod service;
mod types;

pub use service::GlobalOperationsService;
pub use types::{
    BulkDeploymentAction, BulkDeploymentRequest, BulkDeploymentResult, BulkResourceAction,
    BulkResourceItem, BulkResourceKind, BulkResourceRequest, BulkResourceResult, GlobalResourceDto,
    GlobalSearchOptions, ServerDependencyView,
};
