use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::{
    Application as ApplicationProto, ApplicationBuildType, ApplicationMiddleware,
    ApplicationMiddlewareHeader, ApplicationMiddlewareType, ApplicationMount, ApplicationMountType,
    ApplicationOperation, ApplicationPatch, ApplicationPatchType, ApplicationPort,
    ApplicationPortProtocol, ApplicationPortPublishMode, ApplicationRedirect,
    ApplicationSecurityRule, ApplicationSourceType,
};
use crate::services::compute::application::middleware::{
    ApplicationMiddlewareResponseDto, ApplicationMiddlewareType as AppMiddlewareType,
};
use crate::services::compute::application::mount::ApplicationMountResponseDto;
use crate::services::compute::application::patch::{
    ApplicationPatchResponseDto, ApplicationPatchType as AppPatchType,
};
use crate::services::compute::application::port::{
    ApplicationPortResponseDto, PortProtocol, PortPublishMode,
};
use crate::services::compute::application::redirect::ApplicationRedirectResponseDto;
use crate::services::compute::application::security::ApplicationSecurityResponseDto;
use crate::utils::builder::spec::MountType as SpecMountType;

pub fn app_to_proto(value: crate::services::compute::application::ApplicationRecord) -> ApplicationProto {
    use crate::services::compute::database::crud::parse_json_string_vec;
    ApplicationProto {
        id: value.id,
        name: value.name,
        description: value.description,
        app_name: value.app_name,
        source_type: source_type_to_proto(&value.source_type),
        build_type: build_type_to_proto(&value.build_type),
        status: value.app_status,
        last_error: value.last_error,
        trigger_type: value.trigger_type,
        environment_id: value.environment_id,
        server_id: value.server_id,
        build_server_id: value.build_server_id,
        registry_id: value.registry_id,
        network_ids: parse_json_string_vec(&value.network_ids),
        detach_rustploy_network: value.detach_rustploy_network != 0,
        env_var: value.env_var,
        icon: value.icon,
        repository: value.repository,
        owner: value.owner,
        branch: value.branch,
        docker_image: value.docker_image,
        registry_url: value.registry_url,
        custom_git_url: value.custom_git_url,
        custom_git_branch: value.custom_git_branch,
        preview_wildcard: value.preview_wildcard,
        preview_port: value.preview_port,
        created_at: value.created_at,
        updated_at: value.updated_at,
        dockerfile: value.dockerfile,
        docker_context_path: value.docker_context_path,
        docker_build_stage: value.docker_build_stage,
        publish_directory: value.publish_directory,
        is_static_spa: Some(value.is_static_spa),
        railpack_version: value.railpack_version,
        heroku_version: value.heroku_version,
        build_path: value.build_path,
        drop_build_path: value.drop_build_path,
        clean_cache: Some(value.clean_cache),
        auto_deploy: Some(value.auto_deploy),
        memory_reservation: value.memory_reservation,
        memory_limit: value.memory_limit,
        cpu_reservation: value.cpu_reservation,
        cpu_limit: value.cpu_limit,
        replicas: value.replicas,
        command: value.command,
        args: value.args,
        watch_paths: value.watch_paths,
        enable_submodules: Some(value.enable_submodules),
        ..Default::default()
    }
}

pub fn source_type_to_proto(s: &str) -> buffa::EnumValue<ApplicationSourceType> {
    match s {
        "GITHUB" => ApplicationSourceType::Github,
        "GITLAB" => ApplicationSourceType::Gitlab,
        "BITBUCKET" => ApplicationSourceType::Bitbucket,
        "GITEA" => ApplicationSourceType::Gitea,
        // service_git stores the custom-git discriminator as 'CUSTOM';
        // 'CUSTOM_GIT' is accepted for rows written before that.
        "CUSTOM" | "CUSTOM_GIT" => ApplicationSourceType::CustomGit,
        "DOCKER" => ApplicationSourceType::Docker,
        "DROP" => ApplicationSourceType::Drop,
        _ => ApplicationSourceType::Unspecified,
    }
    .into()
}

pub fn build_type_to_proto(s: &str) -> buffa::EnumValue<ApplicationBuildType> {
    match s {
        "BUILDPACKS" => ApplicationBuildType::Buildpacks,
        "DOCKERFILE" => ApplicationBuildType::Dockerfile,
        "NIXPACKS" => ApplicationBuildType::Nixpacks,
        "HEROKU" => ApplicationBuildType::Heroku,
        "PAKETO_BUILDPACKS" => ApplicationBuildType::Paketo,
        "RAILPACK" => ApplicationBuildType::Railpack,
        "STATIC" => ApplicationBuildType::Static,
        _ => ApplicationBuildType::Unspecified,
    }
    .into()
}

pub fn map_db_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("application not found"),
        sqlx::Error::Protocol(msg) if msg.contains("already queued or running") => {
            ConnectError::new(ErrorCode::AlreadyExists, msg.clone())
        }
        sqlx::Error::Protocol(msg) => ConnectError::invalid_argument(msg.clone()),
        sqlx::Error::Database(e) if e.is_foreign_key_violation() => {
            ConnectError::not_found("related resource not found")
        }
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        sqlx::Error::Database(e) if e.is_check_violation() => {
            ConnectError::invalid_argument(e.message().to_string())
        }
        other => {
            tracing::error!(error = %other, "application database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}

pub fn operation_result(
    value: crate::services::compute::application::ApplicationOperationResult,
) -> ApplicationOperation {
    ApplicationOperation {
        application: app_to_proto(value.application).into(),
        operation: value.operation.as_str().into(),
        ..Default::default()
    }
}

// ── Sub-resource entity builders (service DTO → typed proto) ───────

fn protocol_to_proto(s: &str) -> buffa::EnumValue<ApplicationPortProtocol> {
    (match s {
        "UDP" => ApplicationPortProtocol::Udp,
        _ => ApplicationPortProtocol::Tcp,
    })
    .into()
}

fn publish_mode_to_proto(s: &str) -> buffa::EnumValue<ApplicationPortPublishMode> {
    (match s {
        "INGRESS" => ApplicationPortPublishMode::Ingress,
        _ => ApplicationPortPublishMode::Host,
    })
    .into()
}

pub fn port_to_proto(value: ApplicationPortResponseDto) -> ApplicationPort {
    ApplicationPort {
        id: value.id,
        published_port: value.published_port,
        target_port: value.target_port,
        protocol: protocol_to_proto(&value.protocol),
        publish_mode: publish_mode_to_proto(&value.publish_mode),
        application_id: value.application_id,
        created_at: value.created_at,
        ..Default::default()
    }
}

fn mount_type_to_proto(s: &str) -> buffa::EnumValue<ApplicationMountType> {
    (match s {
        "BIND" => ApplicationMountType::Bind,
        "FILE" => ApplicationMountType::File,
        _ => ApplicationMountType::Volume,
    })
    .into()
}

pub fn mount_to_proto(value: ApplicationMountResponseDto) -> ApplicationMount {
    ApplicationMount {
        id: value.id,
        mount_type: mount_type_to_proto(&value.mount_type),
        host_path: value.host_path,
        volume_name: value.volume_name,
        file_path: value.file_path,
        content: value.content,
        mount_path: value.mount_path,
        application_id: value.application_id,
        created_at: value.created_at,
        updated_at: value.updated_at,
        ..Default::default()
    }
}

fn patch_type_to_proto(s: &str) -> buffa::EnumValue<ApplicationPatchType> {
    (match s {
        "UPDATE" => ApplicationPatchType::Update,
        "DELETE" => ApplicationPatchType::Delete,
        _ => ApplicationPatchType::Create,
    })
    .into()
}

pub fn patch_to_proto(value: ApplicationPatchResponseDto) -> ApplicationPatch {
    ApplicationPatch {
        id: value.id,
        patch_type: patch_type_to_proto(&value.patch_type),
        file_path: value.file_path,
        enabled: value.enabled,
        content: value.content,
        application_id: value.application_id,
        created_at: value.created_at,
        updated_at: value.updated_at,
        ..Default::default()
    }
}

fn middleware_type_to_proto(s: &str) -> buffa::EnumValue<ApplicationMiddlewareType> {
    (match s {
        "HEADERS" => ApplicationMiddlewareType::Headers,
        "RATE_LIMIT" => ApplicationMiddlewareType::RateLimit,
        "IP_ALLOWLIST" => ApplicationMiddlewareType::IpAllowlist,
        _ => ApplicationMiddlewareType::Compress,
    })
    .into()
}

pub fn middleware_to_proto(value: ApplicationMiddlewareResponseDto) -> ApplicationMiddleware {
    ApplicationMiddleware {
        id: value.id,
        application_id: value.application_id,
        name: value.name,
        middleware_type: middleware_type_to_proto(&value.middleware_type),
        enabled: value.enabled,
        headers: value
            .headers
            .into_iter()
            .map(|h| ApplicationMiddlewareHeader {
                name: h.name,
                value: h.value,
                ..Default::default()
            })
            .collect(),
        average: value.average,
        burst: value.burst,
        source_ranges: value.source_ranges,
        created_at: value.created_at,
        updated_at: value.updated_at,
        ..Default::default()
    }
}

pub fn redirect_to_proto(value: ApplicationRedirectResponseDto) -> ApplicationRedirect {
    ApplicationRedirect {
        id: value.id,
        regex: value.regex,
        replacement: value.replacement,
        permanent: value.permanent != 0,
        unique_config_key: value.unique_config_key,
        application_id: value.application_id,
        created_at: value.created_at,
        updated_at: value.updated_at,
        ..Default::default()
    }
}

pub fn security_to_proto(value: ApplicationSecurityResponseDto) -> ApplicationSecurityRule {
    ApplicationSecurityRule {
        id: value.id,
        username: value.username,
        has_password: value.has_password,
        application_id: value.application_id,
        created_at: value.created_at,
        ..Default::default()
    }
}

// ── Sub-resource request enums (typed proto → service enum) ────────

pub fn protocol_from_proto(value: buffa::EnumValue<ApplicationPortProtocol>) -> PortProtocol {
    match value {
        buffa::EnumValue::Known(ApplicationPortProtocol::Udp) => PortProtocol::Udp,
        _ => PortProtocol::Tcp,
    }
}

pub fn publish_mode_from_proto(value: buffa::EnumValue<ApplicationPortPublishMode>) -> PortPublishMode {
    match value {
        buffa::EnumValue::Known(ApplicationPortPublishMode::Ingress) => PortPublishMode::Ingress,
        _ => PortPublishMode::Host,
    }
}

pub fn mount_type_from_proto(value: buffa::EnumValue<ApplicationMountType>) -> SpecMountType {
    match value {
        buffa::EnumValue::Known(ApplicationMountType::Bind) => SpecMountType::Bind,
        buffa::EnumValue::Known(ApplicationMountType::File) => SpecMountType::File,
        _ => SpecMountType::Volume,
    }
}

pub fn patch_type_from_proto(value: buffa::EnumValue<ApplicationPatchType>) -> AppPatchType {
    match value {
        buffa::EnumValue::Known(ApplicationPatchType::Update) => AppPatchType::Update,
        buffa::EnumValue::Known(ApplicationPatchType::Delete) => AppPatchType::Delete,
        _ => AppPatchType::Create,
    }
}

pub fn middleware_type_from_proto(
    value: buffa::EnumValue<ApplicationMiddlewareType>,
) -> AppMiddlewareType {
    match value {
        buffa::EnumValue::Known(ApplicationMiddlewareType::Headers) => AppMiddlewareType::Headers,
        buffa::EnumValue::Known(ApplicationMiddlewareType::RateLimit) => {
            AppMiddlewareType::RateLimit
        }
        buffa::EnumValue::Known(ApplicationMiddlewareType::IpAllowlist) => {
            AppMiddlewareType::IpAllowlist
        }
        _ => AppMiddlewareType::Compress,
    }
}
