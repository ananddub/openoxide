use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::{
    OverviewBackupItem as OverviewBackupItemProto,
    OverviewBackupStatus as ProtoOverviewBackupStatus,
    OverviewBackupType as ProtoOverviewBackupType, OverviewDomainItem as OverviewDomainItemProto,
    OverviewServiceItem as OverviewServiceItemProto,
    OverviewServiceType as ProtoOverviewServiceType,
};
use crate::services::platform::overview::{
    OverviewBackupItemDto, OverviewDomainItemDto, OverviewServiceItemDto,
};

pub fn map_db_error(err: sqlx::Error) -> ConnectError {
    tracing::error!(error = %err, "overview database operation failed");
    ConnectError::new(ErrorCode::Internal, "overview database operation failed")
}

fn service_type_to_proto(value: &str) -> ProtoOverviewServiceType {
    match value {
        "APP" => ProtoOverviewServiceType::App,
        "COMPOSE" => ProtoOverviewServiceType::Compose,
        "DATABASE" => ProtoOverviewServiceType::Database,
        _ => ProtoOverviewServiceType::Unspecified,
    }
}

fn backup_type_to_proto(value: &str) -> ProtoOverviewBackupType {
    match value {
        "DATABASE" => ProtoOverviewBackupType::Database,
        "VOLUME" => ProtoOverviewBackupType::Volume,
        _ => ProtoOverviewBackupType::Unspecified,
    }
}

fn backup_status_to_proto(value: &str) -> ProtoOverviewBackupStatus {
    match value {
        "ENABLED" => ProtoOverviewBackupStatus::Enabled,
        "DISABLED" => ProtoOverviewBackupStatus::Disabled,
        _ => ProtoOverviewBackupStatus::Unspecified,
    }
}

pub fn service_item_to_proto(dto: OverviewServiceItemDto) -> OverviewServiceItemProto {
    OverviewServiceItemProto {
        id: dto.id,
        name: dto.name,
        service_type: ::buffa::EnumValue::Known(service_type_to_proto(&dto.service_type)),
        status: dto.status,
        created_at: dto.created_at,
        project_id: dto.project_id,
        project_name: dto.project_name,
        environment_id: dto.environment_id,
        environment_name: dto.environment_name,
        db_kind: dto.db_kind,
        server_id: dto.server_id,
        server_name: dto.server_name,
    }
}

pub fn domain_item_to_proto(dto: OverviewDomainItemDto) -> OverviewDomainItemProto {
    OverviewDomainItemProto {
        id: dto.id,
        host: dto.host,
        path: dto.path,
        port: dto.port,
        https: dto.https,
        application_id: dto.application_id,
        compose_id: dto.compose_id,
        service_name: dto.service_name,
        project_name: dto.project_name,
        internal_path: dto.internal_path,
        custom_entrypoint: dto.custom_entrypoint,
        custom_cert_resolver: dto.custom_cert_resolver,
        strip_path: dto.strip_path,
        middlewares: dto.middlewares,
        domain_type: dto.domain_type,
        certificate_type: dto.certificate_type,
        domain_service_name: dto.domain_service_name,
    }
}

pub fn backup_item_to_proto(dto: OverviewBackupItemDto) -> OverviewBackupItemProto {
    OverviewBackupItemProto {
        id: dto.id,
        name: dto.name,
        backup_type: ::buffa::EnumValue::Known(backup_type_to_proto(&dto.backup_type)),
        status: ::buffa::EnumValue::Known(backup_status_to_proto(&dto.status)),
        destination: dto.destination,
        created_at: dto.created_at,
        service_id: dto.linkage.service_id,
        application_id: dto.linkage.application_id,
        compose_id: dto.linkage.compose_id,
        postgres_id: dto.linkage.postgres_id,
        mysql_id: dto.linkage.mysql_id,
        mariadb_id: dto.linkage.mariadb_id,
        mongo_id: dto.linkage.mongo_id,
        redis_id: dto.linkage.redis_id,
        libsql_id: dto.linkage.libsql_id,
    }
}
