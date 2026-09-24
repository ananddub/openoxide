use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::{
    Settings as SettingsProto, SettingsCertificateType as ProtoCertificateType,
};
use crate::services::platform::settings::{
    SettingsCertificateTypeDto, SettingsResponseDto,
};

pub fn map_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::Protocol(msg) => ConnectError::invalid_argument(msg.clone()),
        other => {
            tracing::error!(error = %other, "settings operation failed");
            ConnectError::new(ErrorCode::Internal, "settings operation failed")
        }
    }
}

pub fn cert_type_to_proto(value: &str) -> ProtoCertificateType {
    match value {
        "NONE" => ProtoCertificateType::None,
        "LETSENCRYPT" => ProtoCertificateType::Letsencrypt,
        "CUSTOM" => ProtoCertificateType::Custom,
        _ => ProtoCertificateType::Unspecified,
    }
}

pub fn proto_to_cert_type(
    value: ::buffa::EnumValue<ProtoCertificateType>,
) -> Result<SettingsCertificateTypeDto, ConnectError> {
    match value {
        ::buffa::EnumValue::Known(ProtoCertificateType::None) => {
            Ok(SettingsCertificateTypeDto::None)
        }
        ::buffa::EnumValue::Known(ProtoCertificateType::Letsencrypt) => {
            Ok(SettingsCertificateTypeDto::Letsencrypt)
        }
        ::buffa::EnumValue::Known(ProtoCertificateType::Custom) => {
            Ok(SettingsCertificateTypeDto::Custom)
        }
        _ => Err(ConnectError::invalid_argument(
            "unsupported certificate type",
        )),
    }
}

pub fn settings_to_proto(dto: SettingsResponseDto) -> SettingsProto {
    SettingsProto {
        id: dto.id,
        server_ip: dto.server_ip,
        certificate_type: ::buffa::EnumValue::Known(cert_type_to_proto(&dto.certificate_type)),
        custom_cert_resolver: dto.custom_cert_resolver,
        https: dto.https,
        host: dto.host,
        lets_encrypt_email: dto.lets_encrypt_email,
        enable_docker_cleanup: dto.enable_docker_cleanup,
        log_cleanup_cron: dto.log_cleanup_cron,
        metrics_config: dto.metrics_config,
        panel_backup_cron: dto.panel_backup_cron,
        log_retention_days: dto.log_retention_days,
        panel_backup_enabled: dto.panel_backup_enabled,
        log_cleanup_enabled: dto.log_cleanup_enabled,
        updated_at: dto.updated_at,
        ..Default::default()
    }
}
