
use buffa::MessageView;
use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    AdvancedConfig, CreateDatabaseRequest, CreateMountRequest, Database as DatabaseProto,
    DatabaseConnection, DatabaseExport,
    DatabaseOperation as DatabaseOperationProto, DatabaseOperationRequest,
    DatabaseService, DatabaseServiceExt, DatabaseTargetRequest, DeleteDatabaseRequest,
    DeleteMountRequest, Dependencies, Empty, ExportDatabaseRequest, GetDatabaseRequest,
    ImportDatabaseRequest, ListDatabasesRequest, ListDatabasesResponse, Mount as MountProto,
    MountList, SwarmSettings, SwarmSettingsRequest, UpdateAdvancedConfigRequest,
    UpdateDatabaseRequest, UpdateSwarmSettingsRequest, UpdateMountRequest,
    ValidationResult,
};
use crate::services::compute::database::crud::{CreateDatabaseDto, PatchDatabaseDto};
use crate::services::compute::database::management::{ImportDatabaseDto, PostgresAdvancedConfigDto};
use crate::services::compute::database::DatabaseOperation;

use super::helper::*;
use super::init::DatabaseController;

#[auto_grpc::service(connect = true, auth = true)]
impl DatabaseService for DatabaseController {
    #[permission(Database, CanRead)]
    async fn get_database<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GetDatabaseRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<DatabaseProto> + Send + use<'a>> {
        let kind = proto_to_kind(request.kind)?;
        let record = self.service.get_by_id(kind, request.id).await.map_err(map_db_error)?;
        Response::ok(record_to_proto(record))
    }

    #[permission(Database, CanRead)]
    async fn list_databases<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ListDatabasesRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListDatabasesResponse> + Send + use<'a>> {
        let kind = proto_to_kind(request.kind)?;
        let items = self
            .service
            .list_by_environment(request.environment_id)
            .await
            .map_err(map_db_error)?;
        let databases = items
            .into_iter()
            .filter(|item| item.kind == kind)
            .map(record_to_proto)
            .collect();
        Response::ok(ListDatabasesResponse { databases, ..Default::default() })
    }

    #[permission(Database, CanCreate)]
    async fn create_database<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, CreateDatabaseRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<DatabaseProto> + Send + use<'a>> {
        let kind = proto_to_kind(request.kind)?;
        let dto = CreateDatabaseDto {
            name: request.name.to_owned(),
            description: request.description.as_deref().map(String::from),
            environment_id: request.environment_id,
            server_id: request.server_id,
            docker_image: request.docker_image.as_deref().map(String::from),
            database_name: request.database_name.as_deref().map(String::from),
            database_user: request.database_user.as_deref().map(String::from),
            database_password: request.database_password.as_deref().map(String::from),
            database_root_password: None,
            external_port: request.external_port,
            external_grpc_port: None,
            external_admin_port: None,
            command: None,
            args: Some(request.args.iter().map(|s| (*s).to_owned()).collect()),
            env_var: request.env_var.as_deref().map(String::from),
            replica_sets: None,
            sqld_node: None,
            sqld_primary_url: None,
            enable_namespaces: None,
            network_ids: Some(request.network_ids.iter().map(|s| (*s).to_owned()).collect()),
            detach_rustploy_network: None,
        };
        let record = self.service.create(kind, dto).await.map_err(map_db_error)?;
        Response::ok(record_to_proto(record))
    }

    #[permission(Database, CanUpdate)]
    async fn update_database<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, UpdateDatabaseRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<DatabaseProto> + Send + use<'a>> {
        let kind = proto_to_kind(request.kind)?;
        let dto = PatchDatabaseDto {
            name: request.name.as_deref().map(String::from),
            description: request.description.as_deref().map(String::from),
            docker_image: request.docker_image.as_deref().map(String::from),
            database_name: request.database_name.as_deref().map(String::from),
            database_user: request.database_user.as_deref().map(String::from),
            database_password: request.database_password.as_deref().map(String::from),
            database_root_password: None,
            external_port: request.external_port,
            external_grpc_port: None,
            external_admin_port: None,
            command: request.command.as_deref().map(String::from),
            args: Some(request.args.iter().map(|s| (*s).to_owned()).collect()),
            env_var: request.env_var.as_deref().map(String::from),
            memory_reservation: request.memory_reservation.as_deref().map(String::from),
            memory_limit: request.memory_limit.as_deref().map(String::from),
            cpu_reservation: request.cpu_reservation.as_deref().map(String::from),
            cpu_limit: request.cpu_limit.as_deref().map(String::from),
            replicas: request.replicas,
            server_id: request.server_id,
            replica_sets: None,
            sqld_node: None,
            sqld_primary_url: None,
            enable_namespaces: None,
            network_ids: Some(request.network_ids.iter().map(|s| (*s).to_owned()).collect()),
            detach_rustploy_network: request.detach_rustploy_network,
        };
        let record = self.service.patch(kind, request.id, dto).await.map_err(map_db_error)?;
        Response::ok(record_to_proto(record))
    }

    #[permission(Database, CanDelete)]
    async fn delete_database<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DeleteDatabaseRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let kind = proto_to_kind(request.kind)?;
        self.service.delete(kind, request.id).await.map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Database, CanUpdate)]
    async fn deploy<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DatabaseOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<DatabaseOperationProto> + Send + use<'a>> {
        let kind = proto_to_kind(request.kind)?;
        let result = run_operation(&self.service, kind, request.id, DatabaseOperation::Deploy, auth.user_id()).await?;
        Response::ok(result)
    }

    #[permission(Database, CanUpdate)]
    async fn redeploy<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DatabaseOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<DatabaseOperationProto> + Send + use<'a>> {
        let kind = proto_to_kind(request.kind)?;
        let result = run_operation(&self.service, kind, request.id, DatabaseOperation::Redeploy, auth.user_id()).await?;
        Response::ok(result)
    }

    #[permission(Database, CanUpdate)]
    async fn reload<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DatabaseOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<DatabaseOperationProto> + Send + use<'a>> {
        let kind = proto_to_kind(request.kind)?;
        let result = run_operation(&self.service, kind, request.id, DatabaseOperation::Reload, auth.user_id()).await?;
        Response::ok(result)
    }

    #[permission(Database, CanUpdate)]
    async fn start<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DatabaseOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<DatabaseOperationProto> + Send + use<'a>> {
        let kind = proto_to_kind(request.kind)?;
        let result = run_operation(&self.service, kind, request.id, DatabaseOperation::Start, auth.user_id()).await?;
        Response::ok(result)
    }

    #[permission(Database, CanUpdate)]
    async fn stop<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DatabaseOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<DatabaseOperationProto> + Send + use<'a>> {
        let kind = proto_to_kind(request.kind)?;
        let result = run_operation(&self.service, kind, request.id, DatabaseOperation::Stop, auth.user_id()).await?;
        Response::ok(result)
    }

    #[permission(Database, CanUpdate)]
    async fn cancel<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DatabaseOperationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let kind = proto_to_kind(request.kind)?;
        self.service
            .cancel_operation(kind, request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Database, CanRead)]
    async fn get_swarm_settings<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, SwarmSettingsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<SwarmSettings> + Send + use<'a>> {
        let kind = proto_to_kind(request.kind)?;
        let dto = self.swarm.get(kind, request.id).await.map_err(map_db_error)?;
        let settings = dto_to_swarm_settings(dto);
        Response::ok(settings)
    }

    #[permission(Database, CanUpdate)]
    async fn update_swarm_settings<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, UpdateSwarmSettingsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<SwarmSettings> + Send + use<'a>> {
        let kind = proto_to_kind(request.kind)?;
        let owned = request
            .settings
            .as_option()
            .map(|view| view.to_owned_message())
            .transpose()
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?
            .unwrap_or_default();
        let dto = swarm_settings_to_dto(&owned);
        self.swarm
            .update(kind, request.id, dto)
            .await
            .map_err(map_db_error)?;
        let settings = dto_to_swarm_settings(swarm_settings_to_dto(&owned));
        Response::ok(settings)
    }

    #[permission(Database, CanRead)]
    async fn list_mounts<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DatabaseTargetRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<MountList> + Send + use<'a>> {
        let kind = proto_to_kind(request.kind)?;
        let items = self.mounts.list(kind, request.id).await.map_err(map_db_error)?;
        let mounts = items.into_iter().map(mount_to_proto).collect();
        Response::ok(MountList { mounts, ..Default::default() })
    }

    #[permission(Database, CanCreate)]
    async fn create_mount<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, CreateMountRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<MountProto> + Send + use<'a>> {
        let kind = proto_to_kind(request.kind)?;
        let mount = self
            .mounts
            .create(kind, request.database_id, request.to_owned_message())
            .await
            .map_err(map_db_error)?;
        Response::ok(mount_to_proto(mount))
    }

    #[permission(Database, CanUpdate)]
    async fn update_mount<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, UpdateMountRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<MountProto> + Send + use<'a>> {
        let kind = proto_to_kind(request.kind)?;
        let mount = self
            .mounts
            .update(kind, request.database_id, request.id, request.to_owned_message())
            .await
            .map_err(map_db_error)?;
        Response::ok(mount_to_proto(mount))
    }

    #[permission(Database, CanDelete)]
    async fn delete_mount<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DeleteMountRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let kind = proto_to_kind(request.kind)?;
        let deleted = self
            .mounts
            .delete(kind, request.database_id, request.id)
            .await
            .map_err(map_db_error)?;
        if deleted {
            Response::ok(Empty::default())
        } else {
            Err(ConnectError::not_found("database mount not found"))
        }
    }

    #[permission(Database, CanRead)]
    async fn list_dependencies<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DatabaseTargetRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Dependencies> + Send + use<'a>> {
        let kind = proto_to_kind(request.kind)?;
        let counts = self.service.dependencies(kind, request.id).await.map_err(map_db_error)?;
        let mut items = Vec::new();
        if counts.active_deployments > 0 { items.push(format!("active_deployments: {}", counts.active_deployments)); }
        if counts.deployments > 0 { items.push(format!("deployments: {}", counts.deployments)); }
        if counts.domains > 0 { items.push(format!("domains: {}", counts.domains)); }
        if counts.mounts > 0 { items.push(format!("mounts: {}", counts.mounts)); }
        if counts.patches > 0 { items.push(format!("patches: {}", counts.patches)); }
        if counts.schedules > 0 { items.push(format!("schedules: {}", counts.schedules)); }
        if counts.backups > 0 { items.push(format!("backups: {}", counts.backups)); }
        if counts.enabled_backups > 0 { items.push(format!("enabled_backups: {}", counts.enabled_backups)); }
        if counts.preview_deployments > 0 { items.push(format!("preview_deployments: {}", counts.preview_deployments)); }
        if counts.middleware > 0 { items.push(format!("middleware: {}", counts.middleware)); }
        if counts.ports > 0 { items.push(format!("ports: {}", counts.ports)); }
        if counts.redirects > 0 { items.push(format!("redirects: {}", counts.redirects)); }
        if counts.security_rules > 0 { items.push(format!("security_rules: {}", counts.security_rules)); }
        Response::ok(Dependencies { items, ..Default::default() })
    }

    #[permission(Database, CanRead)]
    async fn export_database<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ExportDatabaseRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<DatabaseExport> + Send + use<'a>> {
        let kind = proto_to_kind(request.kind)?;
        let bundle = self
            .service
            .export(kind, request.id, request.include_secrets)
            .await
            .map_err(map_db_error)?;
        let archive = serde_json::to_string_pretty(&bundle)
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?;
        Response::ok(DatabaseExport {
            archive,
            ..Default::default()
        })
    }

    #[permission(Database, CanUpdate)]
    async fn import_database<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ImportDatabaseRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<DatabaseProto> + Send + use<'a>> {
        let dto = ImportDatabaseDto {
            archive: request.archive.to_owned(),
            environment_id: request.environment_id,
            server_id: None,
            name: None,
        };
        let record = self.service.import(dto).await.map_err(map_db_error)?;
        Response::ok(record_to_proto(record))
    }

    #[permission(Database, CanRead)]
    async fn get_connection<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DatabaseTargetRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<DatabaseConnection> + Send + use<'a>> {
        let kind = proto_to_kind(request.kind)?;
        let conn = self
            .service
            .connection_details(kind, request.id)
            .await
            .map_err(map_db_error)?;
        Response::ok(DatabaseConnection {
            kind: ::buffa::EnumValue::Known(kind_to_proto(kind)),
            host: conn.host,
            port: conn.port,
            database: conn.database.unwrap_or_default(),
            username: conn.username.unwrap_or_default(),
            ..Default::default()
        })
    }

    #[permission(Database, CanUpdate)]
    async fn rotate_credentials<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DatabaseTargetRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<DatabaseProto> + Send + use<'a>> {
        let kind = proto_to_kind(request.kind)?;
        let _result = self
            .service
            .rotate_credentials(kind, request.id, None)
            .await
            .map_err(map_db_error)?;
        // After rotation, fetch the updated record to return
        let record = self.service.get_by_id(kind, request.id).await.map_err(map_db_error)?;
        Response::ok(record_to_proto(record))
    }

    #[permission(Database, CanRead)]
    async fn validate_database<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DatabaseTargetRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ValidationResult> + Send + use<'a>> {
        let kind = proto_to_kind(request.kind)?;
        let result = self
            .service
            .validate_configuration(kind, request.id)
            .await
            .map_err(map_db_error)?;
        let message = if result.errors.is_empty() && result.warnings.is_empty() {
            None
        } else {
            Some(result.errors.iter().chain(result.warnings.iter()).cloned().collect::<Vec<_>>().join("; "))
        };
        Response::ok(ValidationResult {
            valid: result.valid,
            message,
            ..Default::default()
        })
    }

    #[permission(Database, CanRead)]
    async fn get_postgres_advanced_config<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DatabaseTargetRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<AdvancedConfig> + Send + use<'a>> {
        let config = self
            .service
            .get_postgres_advanced_config(request.id)
            .await
            .map_err(map_db_error)?;
        let replication_role = Some(config.replication_role.as_str().to_string());
        // Fold the replica connection fields back into the settings map so
        // the card can round-trip them (the proto carries one JSON blob).
        let mut settings = config.settings;
        if let Some(host) = &config.primary_host {
            settings.insert("primary_host".into(), host.clone());
        }
        if let Some(port) = config.primary_port {
            settings.insert("primary_port".into(), port.to_string());
        }
        if let Some(user) = &config.replication_user {
            settings.insert("replication_user".into(), user.clone());
        }
        let config_json = serde_json::to_string(&settings)
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?;
        Response::ok(AdvancedConfig {
            replication_role,
            config: Some(config_json),
            ..Default::default()
        })
    }

    #[permission(Database, CanUpdate)]
    async fn update_postgres_advanced_config<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, UpdateAdvancedConfigRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<AdvancedConfig> + Send + use<'a>> {
        let config = request
            .config
            .as_option()
            .map(|view| view.to_owned_message())
            .transpose()
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?
            .unwrap_or_default();
        let replication_role_str = config.replication_role.as_deref().unwrap_or("STANDALONE");
        let replication_role = crate::services::compute::database::management::PostgresReplicationRole::from_str(replication_role_str)
            .ok_or_else(|| ConnectError::invalid_argument(format!("invalid replication role: {}", replication_role_str)))?;
        let config_map: std::collections::BTreeMap<String, String> = config
            .config
            .as_deref()
            .map(|s| serde_json::from_str(s))
            .transpose()
            .map_err(|e| ConnectError::invalid_argument(format!("invalid config JSON: {}", e)))?
            .unwrap_or_default();
        // Replica connection fields ride inside the settings map (the proto
        // carries one JSON blob); lift them into the DTO and strip them from
        // the tunable-settings map (validate_postgres_settings rejects
        // unknown keys). The GET handler folds them back for the card.
        let take_str = |map: &std::collections::BTreeMap<String, String>, key: &str| {
            map.get(key).filter(|v| !v.trim().is_empty()).cloned()
        };
        let mut settings = config_map.clone();
        for key in ["primary_host", "primary_port", "replication_user", "replication_password"] {
            settings.remove(key);
        }
        let dto = PostgresAdvancedConfigDto {
            settings,
            replication_role,
            primary_host: take_str(&config_map, "primary_host"),
            primary_port: config_map
                .get("primary_port")
                .and_then(|v| v.parse::<i64>().ok()),
            replication_user: take_str(&config_map, "replication_user"),
            replication_password: take_str(&config_map, "replication_password"),
        };
        let result = self
            .service
            .update_postgres_advanced_config(request.id, dto)
            .await
            .map_err(map_db_error)?;
        let replication_role_out = Some(result.replication_role.as_str().to_string());
        let config_json = serde_json::to_string(&result.settings)
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?;
        Response::ok(AdvancedConfig {
            replication_role: replication_role_out,
            config: Some(config_json),
            ..Default::default()
        })
    }
}
