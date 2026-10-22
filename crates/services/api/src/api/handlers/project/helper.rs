use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::Project as ProjectProto;

/// Projects are addressed by a global id, so every lookup has to be
/// constrained to the caller's organization. A mismatched org reports
/// not-found rather than permission-denied so ids from other tenants
/// stay indistinguishable from missing ones.
pub fn ensure_project_org(
    project: &crate::flash_gen::models::Projects,
    organization_id: i64,
) -> Result<(), ConnectError> {
    if project.organization_id == organization_id {
        Ok(())
    } else {
        Err(ConnectError::not_found("project not found"))
    }
}

/// Mirrors the `min_len: 1, max_len: 255` constraint on
/// `CreateProjectRequest.name`; buf.validate annotations are not
/// enforced by the Connect router on their own.
pub fn validated_project_name(name: &str) -> Result<String, ConnectError> {
    let name = name.trim();
    if name.is_empty() {
        return Err(ConnectError::invalid_argument("project name is required"));
    }
    if name.chars().count() > 255 {
        return Err(ConnectError::invalid_argument(
            "project name must be at most 255 characters",
        ));
    }
    Ok(name.to_owned())
}

pub fn project_to_proto(p: crate::flash_gen::models::Projects) -> ProjectProto {
    ProjectProto {
        id: p.id,
        name: p.name,
        description: p.description,
        environment_variables: p.env_var,
        organization_id: p.organization_id,
        created_at: p.created_at,
        updated_at: p.updated_at,
    }
}

pub fn map_db_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("project not found"),
        sqlx::Error::Database(e) if e.is_unique_violation() => {
            ConnectError::new(ErrorCode::AlreadyExists, e.message())
        }
        other => {
            tracing::error!(error = %other, "project database operation failed");
            ConnectError::new(ErrorCode::Internal, "database operation failed")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flash_gen::models::Projects;

    fn project(organization_id: i64) -> Projects {
        Projects {
            id: 7,
            name: "website".into(),
            description: None,
            env_var: String::new(),
            organization_id,
            created_at: 0,
            updated_at: 0,
        }
    }

    #[test]
    fn rejects_projects_from_another_organization() {
        assert!(ensure_project_org(&project(2), 1).is_err());
        assert!(ensure_project_org(&project(1), 1).is_ok());
    }

    #[test]
    fn project_name_must_not_be_blank_or_too_long() {
        assert!(validated_project_name("").is_err());
        assert!(validated_project_name("   ").is_err());
        assert_eq!(validated_project_name(" website ").unwrap(), "website");
        assert!(validated_project_name(&"x".repeat(255)).is_ok());
        assert!(validated_project_name(&"x".repeat(256)).is_err());
    }
}
