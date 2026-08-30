use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::{
    Application as ApplicationProto, ApplicationBuildType, ApplicationOperation,
    ApplicationSourceType, ResourceResponse,
};

pub fn app_to_proto(value: crate::services::application::ApplicationRecord) -> ApplicationProto {
    use crate::services::database::crud::parse_json_string_vec;
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
        ..Default::default()
    }
}

pub fn source_type_to_proto(s: &str) -> buffa::EnumValue<ApplicationSourceType> {
    match s {
        "GITHUB" => ApplicationSourceType::Github,
        "GITLAB" => ApplicationSourceType::Gitlab,
        "BITBUCKET" => ApplicationSourceType::Bitbucket,
        "GITEA" => ApplicationSourceType::Gitea,
        "CUSTOM_GIT" => ApplicationSourceType::CustomGit,
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
    value: crate::services::application::ApplicationOperationResult,
) -> ApplicationOperation {
    ApplicationOperation {
        application: app_to_proto(value.application).into(),
        operation: value.operation.as_str().into(),
        ..Default::default()
    }
}

pub fn middleware_to_resource(
    value: crate::services::application::middleware::ApplicationMiddlewareResponseDto,
) -> ResourceResponse {
    let id = value.id;
    let payload = serde_json::to_string(&value).unwrap_or_default();
    ResourceResponse {
        id,
        kind: "application_middleware".into(),
        payload,
        ..Default::default()
    }
}

pub fn mount_to_resource(value: crate::flash_gen::models::Mounts) -> ResourceResponse {
    let dto = crate::services::application::mount::ApplicationMountResponseDto::from(value);
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id,
        kind: "application_mount".into(),
        payload,
        ..Default::default()
    }
}

pub fn patch_to_resource(value: crate::flash_gen::models::Patches) -> ResourceResponse {
    let dto = crate::services::application::patch::ApplicationPatchResponseDto::from(value);
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id,
        kind: "application_patch".into(),
        payload,
        ..Default::default()
    }
}

pub fn port_to_resource(value: crate::flash_gen::models::Ports) -> ResourceResponse {
    let dto = crate::services::application::port::ApplicationPortResponseDto::from(value);
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id,
        kind: "application_port".into(),
        payload,
        ..Default::default()
    }
}

pub fn redirect_to_resource(value: crate::services::application::redirect::Redirect) -> ResourceResponse {
    let dto =
        crate::services::application::redirect::ApplicationRedirectResponseDto::from(value);
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id,
        kind: "application_redirect".into(),
        payload,
        ..Default::default()
    }
}

pub fn security_to_resource(value: crate::flash_gen::models::Security) -> ResourceResponse {
    let dto =
        crate::services::application::security::ApplicationSecurityResponseDto::from(value);
    let payload = serde_json::to_string(&dto).unwrap_or_default();
    ResourceResponse {
        id: dto.id,
        kind: "application_security".into(),
        payload,
        ..Default::default()
    }
}
