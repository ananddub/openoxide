use buffa::MessageView;
use connectrpc::{
    ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult,
};

use crate::api::connect::generated::openoxide::api::v1::{
    ServerActionResult, ServerBackup, ServerCleanupExecution,
    ServerCleanupPolicy as CleanupPolicyProto, ServerCleanupStatus,
    ServerManagement as ServerManagementProto, ServerManagementBackupRequest,
    ServerManagementBackupResponse, ServerManagementConfigureGpuRequest,
    ServerManagementConfigureGpuResponse, ServerManagementDiagnosticsRequest,
    ServerManagementDiagnosticsResponse, ServerManagementGetRequest, ServerManagementGetResponse,
    ServerManagementListCleanupHistoryRequest, ServerManagementListCleanupHistoryResponse,
    ServerManagementRepairAuditRequest, ServerManagementRepairAuditResponse,
    ServerManagementRunCleanupRequest, ServerManagementRunCleanupResponse, ServerManagementService,
    ServerManagementServiceExt, ServerManagementUpdateRequest, ServerManagementUpdateResponse,
    ServerManagementUpgradeRequest, ServerManagementUpgradeResponse,
};

use super::init::ServerManagementController;

fn map_db_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("server management not found"),
        sqlx::Error::Database(e) if e.is_foreign_key_violation() => {
            tracing::warn!(error = %e.message(), "foreign key violation");
            ConnectError::not_found("referenced resource not found")
        }
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            tracing::warn!(error = %e.message(), "unique constraint violation");
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        sqlx::Error::Database(e) if e.is_check_violation() => {
            tracing::warn!(error = %e.message(), "check constraint violation");
            ConnectError::invalid_argument(e.message())
        }
        sqlx::Error::Protocol(msg) => {
            tracing::warn!(error = %msg, "operation protocol/validation error");
            ConnectError::invalid_argument(msg.clone())
        }
        other => {
            tracing::error!(error = %other, "server management database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}

fn cleanup_status_to_proto(status: &str) -> ServerCleanupStatus {
    match status {
        "SUCCEEDED" => ServerCleanupStatus::Succeeded,
        "FAILED" => ServerCleanupStatus::Failed,
        _ => ServerCleanupStatus::Unspecified,
    }
}

// The management/lifecycle/cleanup DTO structs live in private service modules
// and cannot be named from the handler, so map them via expression macros that
// only rely on their public field access.
macro_rules! management_to_proto {
    ($dto:expr) => {{
        let dto = $dto;
        ServerManagementProto {
            server_id: dto.server_id,
            maintenance_mode: dto.maintenance_mode,
            maintenance_message: dto.maintenance_message,
            labels: dto.labels.into_iter().collect(),
            cleanup_policy: ::buffa::MessageField::some(CleanupPolicyProto {
                containers: dto.cleanup_policy.containers,
                images: dto.cleanup_policy.images,
                networks: dto.cleanup_policy.networks,
                volumes: dto.cleanup_policy.volumes,
                packages: dto.cleanup_policy.packages,
            }),
            gpu_enabled: dto.gpu_enabled,
            updated_at: dto.updated_at,
        }
    }};
}

macro_rules! action_result_to_proto {
    ($dto:expr) => {{
        let dto = $dto;
        ServerActionResult {
            success: dto.success,
            stdout: dto.stdout,
            stderr: dto.stderr,
        }
    }};
}

macro_rules! execution_to_proto {
    ($dto:expr) => {{
        let dto = $dto;
        ServerCleanupExecution {
            id: dto.id,
            server_id: dto.server_id,
            status: ::buffa::EnumValue::Known(cleanup_status_to_proto(&dto.status)),
            policy: dto.policy,
            stdout: dto.stdout,
            stderr: dto.stderr,
            started_at: dto.started_at,
            finished_at: dto.finished_at,
        }
    }};
}

#[auto_grpc::service(connect = true, auth = true)]
impl ServerManagementService for ServerManagementController {
    #[permission(Server, CanRead)]
    async fn get_management<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ServerManagementGetRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ServerManagementGetResponse> + Send + use<'a>>
    {
        let management = self
            .management
            .get(request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(ServerManagementGetResponse {
            management: ::buffa::MessageField::some(management_to_proto!(management)),
        })
    }

    #[permission(Server, CanCreate)]
    async fn update_management<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ServerManagementUpdateRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ServerManagementUpdateResponse> + Send + use<'a>>
    {
        // The management service consumes a JSON document; build it from the
        // typed request instead of forwarding a raw envelope payload.
        let policy = request
            .cleanup_policy
            .as_option()
            .map(|view| view.to_owned_message())
            .transpose()
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?
            .unwrap_or_default();
        let payload = serde_json::json!({
            "maintenance_mode": request.maintenance_mode,
            "maintenance_message": request.maintenance_message,
            "labels": request
                .labels
                .iter()
                .map(|(key, value)| {
                    (key.to_string(), serde_json::Value::String(value.to_string()))
                })
                .collect::<serde_json::Map<String, serde_json::Value>>(),
            "cleanup_policy": {
                "containers": policy.containers,
                "images": policy.images,
                "networks": policy.networks,
                "volumes": policy.volumes,
                "packages": policy.packages,
            },
            "gpu_enabled": request.gpu_enabled,
        })
        .to_string();
        let management = self
            .management
            .update(request.id, &payload)
            .await
            .map_err(map_db_error)?;
        Response::ok(ServerManagementUpdateResponse {
            management: ::buffa::MessageField::some(management_to_proto!(management)),
        })
    }

    #[permission(Server, CanCreate)]
    async fn repair_audit<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ServerManagementRepairAuditRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<ServerManagementRepairAuditResponse> + Send + use<'a>,
    > {
        let result = self
            .lifecycle
            .auto_repair(request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(ServerManagementRepairAuditResponse {
            result: ::buffa::MessageField::some(action_result_to_proto!(result)),
        })
    }

    #[permission(Server, CanCreate)]
    async fn configure_gpu<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ServerManagementConfigureGpuRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<ServerManagementConfigureGpuResponse> + Send + use<'a>,
    > {
        let result = self
            .lifecycle
            .configure_gpu(request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(ServerManagementConfigureGpuResponse {
            result: ::buffa::MessageField::some(action_result_to_proto!(result)),
        })
    }

    #[permission(Server, CanCreate)]
    async fn run_cleanup<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ServerManagementRunCleanupRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<ServerManagementRunCleanupResponse> + Send + use<'a>,
    > {
        let result = self.cleanup.run(request.id).await.map_err(map_db_error)?;
        Response::ok(ServerManagementRunCleanupResponse {
            result: ::buffa::MessageField::some(action_result_to_proto!(result)),
        })
    }

    #[permission(Server, CanRead)]
    async fn list_cleanup_history<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ServerManagementListCleanupHistoryRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<ServerManagementListCleanupHistoryResponse> + Send + use<'a>,
    > {
        let executions = self
            .cleanup
            .history(request.server_id)
            .await
            .map_err(map_db_error)?
            .into_iter()
            .map(|dto| execution_to_proto!(dto))
            .collect();
        Response::ok(ServerManagementListCleanupHistoryResponse { executions })
    }

    #[permission(Server, CanCreate)]
    async fn upgrade<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ServerManagementUpgradeRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ServerManagementUpgradeResponse> + Send + use<'a>>
    {
        let result = self
            .lifecycle
            .upgrade(request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(ServerManagementUpgradeResponse {
            result: ::buffa::MessageField::some(action_result_to_proto!(result)),
        })
    }

    #[permission(Server, CanCreate)]
    async fn backup<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ServerManagementBackupRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ServerManagementBackupResponse> + Send + use<'a>>
    {
        let backup = self
            .lifecycle
            .backup(request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(ServerManagementBackupResponse {
            backup: ::buffa::MessageField::some(ServerBackup {
                remote_path: backup.remote_path,
                created_at: backup.created_at,
            }),
        })
    }

    #[permission(Server, CanRead)]
    async fn diagnostics<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ServerManagementDiagnosticsRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<ServerManagementDiagnosticsResponse> + Send + use<'a>,
    > {
        let result = self
            .lifecycle
            .diagnostics(request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(ServerManagementDiagnosticsResponse {
            result: ::buffa::MessageField::some(action_result_to_proto!(result)),
        })
    }
}
