
use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::{
    Database as DatabaseProto, DatabaseKind as ProtoDatabaseKind,
    DatabaseOperation as DatabaseOperationProto, Mount as MountProto, SwarmSettings,
};
use crate::services::compute::database::crud::parse_json_string_vec;
use crate::services::compute::database::swarm_settings::DatabaseSwarmSettingsDto;
use crate::services::compute::database::{
    DatabaseKind, DatabaseOperation, DatabaseService as AppDatabaseService,
};

pub fn kind_to_proto(kind: DatabaseKind) -> ProtoDatabaseKind {
    match kind {
        DatabaseKind::Postgres => ProtoDatabaseKind::Postgres,
        DatabaseKind::Mysql => ProtoDatabaseKind::Mysql,
        DatabaseKind::Mariadb => ProtoDatabaseKind::Mariadb,
        DatabaseKind::Mongo => ProtoDatabaseKind::Mongo,
        DatabaseKind::Redis => ProtoDatabaseKind::Redis,
        DatabaseKind::Libsql => ProtoDatabaseKind::Libsql,
    }
}

pub fn proto_to_kind(value: ::buffa::EnumValue<ProtoDatabaseKind>) -> Result<DatabaseKind, ConnectError> {
    match value {
        ::buffa::EnumValue::Known(ProtoDatabaseKind::Postgres) => Ok(DatabaseKind::Postgres),
        ::buffa::EnumValue::Known(ProtoDatabaseKind::Mysql) => Ok(DatabaseKind::Mysql),
        ::buffa::EnumValue::Known(ProtoDatabaseKind::Mariadb) => Ok(DatabaseKind::Mariadb),
        ::buffa::EnumValue::Known(ProtoDatabaseKind::Mongo) => Ok(DatabaseKind::Mongo),
        ::buffa::EnumValue::Known(ProtoDatabaseKind::Redis) => Ok(DatabaseKind::Redis),
        ::buffa::EnumValue::Known(ProtoDatabaseKind::Libsql) => Ok(DatabaseKind::Libsql),
        _ => Err(ConnectError::invalid_argument("unsupported database kind")),
    }
}

pub fn record_to_proto(record: crate::services::compute::database::DatabaseRecord) -> DatabaseProto {
    DatabaseProto {
        kind: ::buffa::EnumValue::Known(kind_to_proto(record.kind)),
        id: record.id,
        name: record.name,
        description: record.description,
        docker_image: record.docker_image,
        database_name: record.database_name,
        database_user: record.database_user,
        database_password: record.database_password,
        external_port: record.external_port,
        command: record.command,
        args: parse_json_string_vec(record.args.as_deref().unwrap_or("[]")),
        env_var: record.env_var,
        memory_limit: record.memory_limit,
        cpu_limit: record.cpu_limit,
        replicas: record.replicas,
        network_ids: parse_json_string_vec(&record.network_ids),
        status: record.app_status.clone(),
        last_error: record.last_error,
        environment_id: record.environment_id,
        server_id: record.server_id,
        created_at: record.created_at,
        updated_at: record.updated_at,
        ..Default::default()
    }
}

pub fn mount_to_proto(mount: crate::flash_gen::models::Mounts) -> MountProto {
    MountProto {
        id: mount.id,
        mount_type: mount.mount_type,
        host_path: mount.host_path,
        volume_name: mount.volume_name,
        file_path: mount.file_path,
        content: mount.content,
        mount_path: mount.mount_path,
        created_at: mount.created_at,
        updated_at: mount.updated_at,
        ..Default::default()
    }
}

pub fn map_db_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("database not found"),
        sqlx::Error::Database(e) if e.is_foreign_key_violation() => {
            ConnectError::not_found("related resource not found")
        }
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        sqlx::Error::Database(e) if e.is_check_violation() => {
            ConnectError::invalid_argument(e.message().to_string())
        }
        sqlx::Error::Protocol(msg) => ConnectError::invalid_argument(msg.clone()),
        other => {
            tracing::error!(error = %other, "database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}

pub async fn run_operation(
    service: &AppDatabaseService,
    kind: DatabaseKind,
    id: i64,
    operation: DatabaseOperation,
    actor_id: i64,
) -> Result<DatabaseOperationProto, ConnectError> {
    let result = service
        .run_operation(kind, id, operation, actor_id)
        .await
        .map_err(map_db_error)?;
    Ok(DatabaseOperationProto {
        database: ::buffa::MessageField::some(record_to_proto(result.database)),
        operation: result.operation.as_str().into(),
        ..Default::default()
    })
}

pub fn dto_to_swarm_settings(dto: DatabaseSwarmSettingsDto) -> SwarmSettings {
    SwarmSettings {
        health_check: dto.health_check_swarm,
        restart_policy: dto.restart_policy_swarm,
        placement: dto.placement_swarm,
        update_config: dto.update_config_swarm,
        rollback_config: dto.rollback_config_swarm,
        mode: dto.mode_swarm,
        labels: dto.labels_swarm,
        network: dto.network_swarm,
        endpoint_spec: dto.endpoint_spec_swarm,
        ulimits: dto.ulimits_swarm,
        stop_grace_period: dto.stop_grace_period_swarm,
        ..Default::default()
    }
}

pub fn swarm_settings_to_dto(settings: &SwarmSettings) -> DatabaseSwarmSettingsDto {
    DatabaseSwarmSettingsDto {
        health_check_swarm: settings.health_check.clone(),
        restart_policy_swarm: settings.restart_policy.clone(),
        placement_swarm: settings.placement.clone(),
        update_config_swarm: settings.update_config.clone(),
        rollback_config_swarm: settings.rollback_config.clone(),
        mode_swarm: settings.mode.clone(),
        labels_swarm: settings.labels.clone(),
        network_swarm: settings.network.clone(),
        endpoint_spec_swarm: settings.endpoint_spec.clone(),
        ulimits_swarm: settings.ulimits.clone(),
        stop_grace_period_swarm: settings.stop_grace_period,
    }
}
