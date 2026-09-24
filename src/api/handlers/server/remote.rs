use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};
use validator::Validate;

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, RemoteServer, RemoteServerActivateRequest, RemoteServerActivateResponse,
    RemoteServerAction, RemoteServerApplicationMigration,
    RemoteServerApplicationMigrationStatus, RemoteServerCancelMigrationRequest,
    RemoteServerCancelMigrationResponse, RemoteServerCreateRequest, RemoteServerCreateResponse,
    RemoteServerDeleteRequest, RemoteServerDeactivateRequest, RemoteServerDeactivateResponse,
    RemoteServerDependencyMigration,
    RemoteServerDependencyMigrationStatus, RemoteServerGetMigrationRequest,
    RemoteServerGetMigrationResponse, RemoteServerGetRequest, RemoteServerGetResponse,
    RemoteServerListMigrationObjectsRequest, RemoteServerListMigrationObjectsResponse,
    RemoteServerListMigrationsRequest, RemoteServerListMigrationsResponse, RemoteServerListRequest,
    RemoteServerListResponse, RemoteServerMigrateDependenciesRequest,
    RemoteServerMigrateDependenciesResponse, RemoteServerMigrateServiceRequest,
    RemoteServerMigrateServiceResponse, RemoteServerMigrationObject,
    RemoteServerMigrationObjectKind, RemoteServerMigrationPhase, RemoteServerRollbackMigrationRequest,
    RemoteServerRollbackMigrationResponse, RemoteServerService, RemoteServerServiceExt,
    RemoteServerStatus as ProtoRemoteServerStatus, RemoteServerTestConnectionRequest,
    RemoteServerTestConnectionResponse, RemoteServerType as ProtoRemoteServerType,
    RemoteServerUpdateRequest, RemoteServerUpdateResponse,
};
use crate::flash_gen::models::{ApplicationMigrations, MigrationObjects, Servers};
use crate::services::infrastructure::server::{
    CreateRemoteServerDto, PatchRemoteServerDto,
    RemoteServerStatusDto as ServiceRemoteServerStatusDto, RemoteServerStatus as ServiceRemoteServerStatus,
    RemoteServerTypeDto as ServiceRemoteServerTypeDto, ServerDependencyMigrationDto,
};

use super::init::RemoteServerController;

fn map_db_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("remote server not found"),
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
            tracing::error!(error = %other, "remote server database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}

// ── DB string ↔ generated enum mappings ────────────────────────────────

fn status_to_proto(status: &str) -> ProtoRemoteServerStatus {
    match status {
        "ACTIVE" => ProtoRemoteServerStatus::Active,
        "INACTIVE" => ProtoRemoteServerStatus::Inactive,
        _ => ProtoRemoteServerStatus::Unspecified,
    }
}

fn server_type_to_proto(server_type: &str) -> ProtoRemoteServerType {
    match server_type {
        "DEPLOY" => ProtoRemoteServerType::Deploy,
        "BUILD" => ProtoRemoteServerType::Build,
        _ => ProtoRemoteServerType::Unspecified,
    }
}

fn dependency_status_to_proto(status: &str) -> RemoteServerDependencyMigrationStatus {
    match status {
        "RUNNING" => RemoteServerDependencyMigrationStatus::Running,
        "SUCCEEDED" => RemoteServerDependencyMigrationStatus::Succeeded,
        "FAILED" => RemoteServerDependencyMigrationStatus::Failed,
        "ROLLED_BACK" => RemoteServerDependencyMigrationStatus::RolledBack,
        _ => RemoteServerDependencyMigrationStatus::Unspecified,
    }
}

fn application_status_to_proto(status: &str) -> RemoteServerApplicationMigrationStatus {
    match status {
        "RUNNING" => RemoteServerApplicationMigrationStatus::Running,
        "APPLYING" => RemoteServerApplicationMigrationStatus::Applying,
        "SUCCEEDED" => RemoteServerApplicationMigrationStatus::Succeeded,
        "FAILED" => RemoteServerApplicationMigrationStatus::Failed,
        "CANCELLED" => RemoteServerApplicationMigrationStatus::Cancelled,
        _ => RemoteServerApplicationMigrationStatus::Unspecified,
    }
}

fn phase_to_proto(phase: &str) -> RemoteServerMigrationPhase {
    match phase {
        "PREFLIGHT" => RemoteServerMigrationPhase::Preflight,
        "QUIESCE" => RemoteServerMigrationPhase::Quiesce,
        "SYNC" => RemoteServerMigrationPhase::Sync,
        "VERIFY" => RemoteServerMigrationPhase::Verify,
        "APPLY" => RemoteServerMigrationPhase::Apply,
        "REPOINT" => RemoteServerMigrationPhase::Repoint,
        "REDEPLOY" => RemoteServerMigrationPhase::Redeploy,
        "HEALTH" => RemoteServerMigrationPhase::Health,
        "SUPERSEDED" => RemoteServerMigrationPhase::Superseded,
        "FAILED" => RemoteServerMigrationPhase::Failed,
        "ROLLED_BACK" => RemoteServerMigrationPhase::RolledBack,
        "CANCELLED" => RemoteServerMigrationPhase::Cancelled,
        "DONE" => RemoteServerMigrationPhase::Done,
        _ => RemoteServerMigrationPhase::Unspecified,
    }
}

fn object_kind_to_proto(kind: &str) -> RemoteServerMigrationObjectKind {
    match kind {
        "VOLUME" => RemoteServerMigrationObjectKind::Volume,
        "BIND" => RemoteServerMigrationObjectKind::Bind,
        "DBDATA" => RemoteServerMigrationObjectKind::Dbdata,
        "DUMP" => RemoteServerMigrationObjectKind::Dump,
        "SRC" => RemoteServerMigrationObjectKind::Src,
        "CERT" => RemoteServerMigrationObjectKind::Cert,
        _ => RemoteServerMigrationObjectKind::Unspecified,
    }
}

fn proto_to_server_type(
    value: ::buffa::EnumValue<ProtoRemoteServerType>,
) -> Result<ServiceRemoteServerTypeDto, ConnectError> {
    match value {
        ::buffa::EnumValue::Known(ProtoRemoteServerType::Deploy) => {
            Ok(ServiceRemoteServerTypeDto::Deploy)
        }
        ::buffa::EnumValue::Known(ProtoRemoteServerType::Build) => {
            Ok(ServiceRemoteServerTypeDto::Build)
        }
        _ => Err(ConnectError::invalid_argument("unsupported server type")),
    }
}

fn proto_to_server_status(
    value: ::buffa::EnumValue<ProtoRemoteServerStatus>,
) -> Result<ServiceRemoteServerStatusDto, ConnectError> {
    match value {
        ::buffa::EnumValue::Known(ProtoRemoteServerStatus::Active) => {
            Ok(ServiceRemoteServerStatusDto::Active)
        }
        ::buffa::EnumValue::Known(ProtoRemoteServerStatus::Inactive) => {
            Ok(ServiceRemoteServerStatusDto::Inactive)
        }
        _ => Err(ConnectError::invalid_argument("unsupported server status")),
    }
}

// ── Owned DB row / service DTO → typed proto mappings ──────────────────

fn server_to_proto(server: Servers) -> RemoteServer {
    RemoteServer {
        id: server.id,
        name: server.name,
        description: server.description,
        ip_address: server.ip_address,
        port: server.port,
        username: server.username,
        app_name: server.app_name,
        server_status: ::buffa::EnumValue::Known(status_to_proto(&server.server_status)),
        server_type: ::buffa::EnumValue::Known(server_type_to_proto(&server.server_type)),
        enable_docker_cleanup: server.enable_docker_cleanup != 0,
        log_cleanup_cron: server.log_cleanup_cron,
        command: server.command,
        metrics_config: server.metrics_config,
        ssh_key_id: server.ssh_key_id,
        created_at: server.created_at,
        updated_at: server.updated_at,
        build_memory_limit: server.build_memory_limit,
        build_cpu_limit: server.build_cpu_limit,
        ..Default::default()
    }
}

fn dependency_migration_to_proto(dto: ServerDependencyMigrationDto) -> RemoteServerDependencyMigration {
    RemoteServerDependencyMigration {
        migration_id: dto.migration_id,
        status: ::buffa::EnumValue::Known(dependency_status_to_proto(&dto.status)),
        source_server_id: dto.source_server_id,
        target_server_id: dto.target_server_id,
        applications: dto.applications,
        build_assignments: dto.build_assignments,
        compose_projects: dto.compose_projects,
        databases: dto.databases,
        certificates: dto.certificates,
        schedules: dto.schedules,
        queued_applications: dto.queued_applications,
        queued_compose_projects: dto.queued_compose_projects,
        error: dto.error,
        ..Default::default()
    }
}

fn application_migration_to_proto(row: ApplicationMigrations) -> RemoteServerApplicationMigration {
    RemoteServerApplicationMigration {
        id: row.id,
        organization_id: row.organization_id,
        service_id: row.service_id,
        source_server_id: row.source_server_id,
        target_server_id: row.target_server_id,
        status: ::buffa::EnumValue::Known(application_status_to_proto(&row.status)),
        phase: ::buffa::EnumValue::Known(phase_to_proto(&row.phase)),
        progress: row.progress,
        cancel_requested: row.cancel_requested != 0,
        error: row.error,
        log_path: row.log_path,
        created_at: row.created_at,
        updated_at: row.updated_at,
        ..Default::default()
    }
}

fn migration_object_to_proto(row: MigrationObjects) -> RemoteServerMigrationObject {
    RemoteServerMigrationObject {
        migration_id: row.migration_id,
        obj_key: row.obj_key,
        kind: ::buffa::EnumValue::Known(object_kind_to_proto(&row.kind)),
        source_ref: row.source_ref,
        dest_ref: row.dest_ref,
        status: row.status,
        bytes: row.bytes,
        error: row.error,
        created_at: row.created_at,
        updated_at: row.updated_at,
        ..Default::default()
    }
}

#[auto_grpc::service(connect = true, auth = true)]
impl RemoteServerService for RemoteServerController {
    #[permission(Server, CanRead)]
    async fn list_remote_servers<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, RemoteServerListRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<RemoteServerListResponse> + Send + use<'a>> {
        let servers = self
            .service
            .list(auth.organization_id)
            .await
            .map_err(map_db_error)?
            .into_iter()
            .map(server_to_proto)
            .collect();
        Response::ok(RemoteServerListResponse {
            servers,
            ..Default::default()
        })
    }

    #[permission(Server, CanRead)]
    async fn get_remote_server<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, RemoteServerGetRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<RemoteServerGetResponse> + Send + use<'a>> {
        let server = self
            .service
            .get_in_org(request.id, auth.organization_id)
            .await
            .map_err(map_db_error)?;
        Response::ok(RemoteServerGetResponse {
            server: ::buffa::MessageField::some(server_to_proto(server)),
            ..Default::default()
        })
    }

    #[permission(Server, CanCreate)]
    async fn create_remote_server<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, RemoteServerCreateRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<RemoteServerCreateResponse> + Send + use<'a>> {
        // Preserve the serde defaults the retired JSON envelope carried:
        // missing port → 22, empty username → "root", unspecified type → DEPLOY.
        let server_type = match request.server_type {
            ::buffa::EnumValue::Known(ProtoRemoteServerType::Unspecified) => {
                ServiceRemoteServerTypeDto::Deploy
            }
            value => proto_to_server_type(value)?,
        };
        let dto = CreateRemoteServerDto {
            name: request.name.to_owned(),
            description: request.description.map(str::to_owned).filter(|value| !value.is_empty()),
            ip_address: request.ip_address.to_owned(),
            port: if request.port == 0 { 22 } else { request.port },
            username: if request.username.is_empty() {
                "root".to_owned()
            } else {
                request.username.to_owned()
            },
            server_type,
            ssh_key_id: request.ssh_key_id,
            build_memory_limit: request.build_memory_limit.map(str::to_owned),
            build_cpu_limit: request.build_cpu_limit.map(str::to_owned),
        };
        dto.validate()
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let server = self
            .service
            .create(dto, auth.organization_id)
            .await
            .map_err(map_db_error)?;
        Response::ok(RemoteServerCreateResponse {
            server: ::buffa::MessageField::some(server_to_proto(server)),
            ..Default::default()
        })
    }

    #[permission(Server, CanUpdate)]
    async fn update_remote_server<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, RemoteServerUpdateRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<RemoteServerUpdateResponse> + Send + use<'a>> {
        let dto = PatchRemoteServerDto {
            name: request.name.map(str::to_owned),
            description: request.description.map(str::to_owned),
            ip_address: request.ip_address.map(str::to_owned),
            port: request.port,
            username: request.username.map(str::to_owned),
            server_status: request
                .server_status
                .map(proto_to_server_status)
                .transpose()?,
            server_type: request
                .server_type
                .map(proto_to_server_type)
                .transpose()?,
            enable_docker_cleanup: request.enable_docker_cleanup.map(i64::from),
            log_cleanup_cron: request.log_cleanup_cron.map(str::to_owned),
            command: request.command.map(str::to_owned),
            metrics_config: request.metrics_config.map(str::to_owned),
            ssh_key_id: request.ssh_key_id,
            build_memory_limit: request.build_memory_limit.map(str::to_owned),
            build_cpu_limit: request.build_cpu_limit.map(str::to_owned),
        };
        dto.validate()
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let server = self
            .service
            .patch(request.id, dto, auth.organization_id)
            .await
            .map_err(map_db_error)?;
        Response::ok(RemoteServerUpdateResponse {
            server: ::buffa::MessageField::some(server_to_proto(server)),
            ..Default::default()
        })
    }

    #[permission(Server, CanCreate)]
    async fn activate<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, RemoteServerActivateRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<RemoteServerActivateResponse> + Send + use<'a>>
    {
        let server = self
            .service
            .set_status(request.id, ServiceRemoteServerStatus::Active)
            .await
            .map_err(map_db_error)?;
        Response::ok(RemoteServerActivateResponse {
            server: ::buffa::MessageField::some(server_to_proto(server)),
            action: ::buffa::EnumValue::Known(RemoteServerAction::Activate),
            ..Default::default()
        })
    }

    #[permission(Server, CanCreate)]
    async fn deactivate<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, RemoteServerDeactivateRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<RemoteServerDeactivateResponse> + Send + use<'a>,
    > {
        let server = self
            .service
            .set_status(request.id, ServiceRemoteServerStatus::Inactive)
            .await
            .map_err(map_db_error)?;
        Response::ok(RemoteServerDeactivateResponse {
            server: ::buffa::MessageField::some(server_to_proto(server)),
            action: ::buffa::EnumValue::Known(RemoteServerAction::Deactivate),
            ..Default::default()
        })
    }

    #[permission(Server, CanRead)]
    async fn test_connection<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, RemoteServerTestConnectionRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<RemoteServerTestConnectionResponse> + Send + use<'a>,
    > {
        let server = self
            .service
            .touch_test_connection(request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(RemoteServerTestConnectionResponse {
            server: ::buffa::MessageField::some(server_to_proto(server)),
            action: ::buffa::EnumValue::Known(RemoteServerAction::TestConnection),
            ..Default::default()
        })
    }

    #[permission(Server, CanDelete)]
    async fn delete_remote_server<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, RemoteServerDeleteRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service
            .delete(request.id, auth.organization_id)
            .await
            .map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Server, CanCreate)]
    async fn migrate_dependencies<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, RemoteServerMigrateDependenciesRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<RemoteServerMigrateDependenciesResponse> + Send + use<'a>,
    > {
        if request.target_server_id < 1 {
            return Err(ConnectError::invalid_argument("target_server_id is required"));
        }
        let migration = self
            .service
            .migrate_dependencies(request.id, request.target_server_id)
            .await
            .map_err(map_db_error)?;
        Response::ok(RemoteServerMigrateDependenciesResponse {
            migration: ::buffa::MessageField::some(dependency_migration_to_proto(migration)),
            ..Default::default()
        })
    }

    #[permission(Server, CanRead)]
    async fn get_migration<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, RemoteServerGetMigrationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<RemoteServerGetMigrationResponse> + Send + use<'a>> {
        let migration = self
            .service
            .migration_status(request.migration_id)
            .await
            .map_err(map_db_error)?;
        Response::ok(RemoteServerGetMigrationResponse {
            migration: ::buffa::MessageField::some(dependency_migration_to_proto(migration)),
            ..Default::default()
        })
    }

    #[permission(Server, CanCreate)]
    async fn rollback_migration<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, RemoteServerRollbackMigrationRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<RemoteServerRollbackMigrationResponse> + Send + use<'a>,
    > {
        let migration = self
            .service
            .rollback_migration(request.migration_id)
            .await
            .map_err(map_db_error)?;
        Response::ok(RemoteServerRollbackMigrationResponse {
            migration: ::buffa::MessageField::some(dependency_migration_to_proto(migration)),
            ..Default::default()
        })
    }

    #[permission(Server, CanCreate)]
    async fn cancel_migration<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, RemoteServerCancelMigrationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<RemoteServerCancelMigrationResponse> + Send + use<'a>> {
        let row = self
            .service
            .request_stateful_migration_cancel(request.migration_id, auth.organization_id)
            .await
            .map_err(map_db_error)?;
        Response::ok(RemoteServerCancelMigrationResponse {
            migration: ::buffa::MessageField::some(application_migration_to_proto(row)),
            ..Default::default()
        })
    }

    #[permission(Server, CanCreate)]
    async fn migrate_service<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, RemoteServerMigrateServiceRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<RemoteServerMigrateServiceResponse> + Send + use<'a>,
    > {
        if request.target_server_id < 1 {
            return Err(ConnectError::invalid_argument("target_server_id is required"));
        }
        let row = self
            .service
            .migrate_service(request.id, request.target_server_id, auth.organization_id)
            .await
            .map_err(map_db_error)?;
        Response::ok(RemoteServerMigrateServiceResponse {
            migration: ::buffa::MessageField::some(application_migration_to_proto(row)),
            ..Default::default()
        })
    }

    #[permission(Server, CanRead)]
    async fn list_migrations<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, RemoteServerListMigrationsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<RemoteServerListMigrationsResponse> + Send + use<'a>> {
        let migrations = self
            .service
            .list_service_migrations(request.service_id, auth.organization_id)
            .await
            .map_err(map_db_error)?
            .into_iter()
            .map(application_migration_to_proto)
            .collect();
        Response::ok(RemoteServerListMigrationsResponse {
            migrations,
            ..Default::default()
        })
    }

    #[permission(Server, CanRead)]
    async fn list_migration_objects<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, RemoteServerListMigrationObjectsRequest>,
    ) -> ServiceResult<
        impl connectrpc::Encodable<RemoteServerListMigrationObjectsResponse> + Send + use<'a>,
    > {
        let objects = self
            .service
            .list_migration_objects(request.migration_id, auth.organization_id)
            .await
            .map_err(map_db_error)?
            .into_iter()
            .map(migration_object_to_proto)
            .collect();
        Response::ok(RemoteServerListMigrationObjectsResponse {
            objects,
            ..Default::default()
        })
    }
}
