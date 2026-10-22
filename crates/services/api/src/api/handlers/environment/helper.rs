use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::Environment as EnvironmentProto;
use crate::services::workspace::environment::EnvironmentService;
use crate::services::workspace::project::ProjectService;

/// Environments are addressed by global ids and carry their project's
/// environment variables, so access must be constrained to a project in
/// the caller's organization. Mismatches report not-found to avoid
/// confirming that another tenant's ids exist.
pub async fn ensure_project_org(project_id: i64, organization_id: i64) -> Result<(), ConnectError> {
    let projects = auto_di::resolve::<ProjectService>()
        .await
        .map_err(|error| {
            tracing::error!(%error, "project service unavailable");
            ConnectError::new(ErrorCode::Internal, "project service unavailable")
        })?;
    match projects.get_by_id(project_id).await {
        Ok(project) if project.organization_id == organization_id => Ok(()),
        Ok(_) | Err(sqlx::Error::RowNotFound) => Err(ConnectError::not_found("project not found")),
        Err(other) => {
            tracing::error!(error = %other, "project lookup failed");
            Err(ConnectError::new(
                ErrorCode::Internal,
                "database operation failed",
            ))
        }
    }
}

pub async fn ensure_environment_org(
    environment: &crate::flash_gen::models::Environments,
    organization_id: i64,
) -> Result<(), ConnectError> {
    ensure_project_org(environment.project_id, organization_id).await
}

/// Loads an environment by id and checks it belongs to the caller's
/// organization. Used by services whose rows only carry `environment_id`.
pub async fn ensure_environment_id_org(
    environment_id: i64,
    organization_id: i64,
) -> Result<(), ConnectError> {
    let environments = auto_di::resolve::<EnvironmentService>()
        .await
        .map_err(|error| {
            tracing::error!(%error, "environment service unavailable");
            ConnectError::new(ErrorCode::Internal, "environment service unavailable")
        })?;
    match environments.get_by_id(environment_id).await {
        Ok(environment) => ensure_environment_org(&environment, organization_id).await,
        Err(sqlx::Error::RowNotFound) => Err(ConnectError::not_found("environment not found")),
        Err(other) => {
            tracing::error!(error = %other, "environment lookup failed");
            Err(ConnectError::new(
                ErrorCode::Internal,
                "database operation failed",
            ))
        }
    }
}

pub fn validated_environment_name(name: &str) -> Result<String, ConnectError> {
    let name = name.trim();
    if name.is_empty() {
        return Err(ConnectError::invalid_argument(
            "environment name is required",
        ));
    }
    if name.chars().count() > 255 {
        return Err(ConnectError::invalid_argument(
            "environment name must be at most 255 characters",
        ));
    }
    Ok(name.to_owned())
}

pub fn env_to_proto(e: crate::flash_gen::models::Environments) -> EnvironmentProto {
    EnvironmentProto {
        id: e.id,
        name: e.name,
        description: e.description,
        environment_variables: e.env_var,
        is_default: e.is_default != 0,
        project_id: e.project_id,
        created_at: e.created_at,
        updated_at: e.updated_at,
    }
}

pub fn map_db_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("environment not found"),
        sqlx::Error::Database(e) if e.is_foreign_key_violation() => {
            ConnectError::not_found("project not found")
        }
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        sqlx::Error::Protocol(msg) => ConnectError::invalid_argument(msg.clone()),
        other => {
            tracing::error!(error = %other, "environment database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn environment_name_must_not_be_blank_or_too_long() {
        assert!(validated_environment_name("").is_err());
        assert!(validated_environment_name(" \t ").is_err());
        assert_eq!(validated_environment_name(" staging ").unwrap(), "staging");
        assert!(validated_environment_name(&"x".repeat(255)).is_ok());
        assert!(validated_environment_name(&"x".repeat(256)).is_err());
    }
}
