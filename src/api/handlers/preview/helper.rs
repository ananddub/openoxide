use connectrpc::{ConnectError, ErrorCode};

use crate::api::connect::generated::openoxide::api::v1::PreviewDeployment;
use crate::services::compute::preview::{PreviewDeploymentOutcome, PreviewDeploymentView};
use crate::services::integrations::git::webhooks::{GitProviderKind, PullRequestEvent};

pub fn view_to_proto(v: PreviewDeploymentView) -> PreviewDeployment {
    PreviewDeployment {
        id: v.id,
        application_id: v.base_application_id,
        status: v.status,
        branch: Some(v.source_branch.clone()),
        url: if v.domain.is_empty() { None } else { Some(v.domain) },
        error: None,
        created_at: v.created_at,
        updated_at: v.updated_at,
        ..Default::default()
    }
}

pub fn outcome_to_proto(o: PreviewDeploymentOutcome) -> PreviewDeployment {
    PreviewDeployment {
        id: o.preview.id,
        application_id: o.preview.base_application_id,
        status: o.preview.status,
        branch: Some(o.preview.source_branch.clone()),
        url: if o.preview.domain.is_empty() { None } else { Some(o.preview.domain) },
        error: None,
        created_at: o.preview.created_at,
        updated_at: o.preview.updated_at,
        ..Default::default()
    }
}

pub fn map_sqlx(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("preview deployment not found"),
        other => {
            tracing::error!(error = %other, "preview deployment database operation failed");
            ConnectError::new(ErrorCode::Internal, "preview deployment operation failed")
        }
    }
}

pub fn map_service(err: String) -> ConnectError {
    let lower = err.to_ascii_lowercase();
    if lower.contains("not found") {
        ConnectError::not_found(err)
    } else if err.contains("already queued or running") {
        ConnectError::new(ErrorCode::AlreadyExists, err)
    } else {
        ConnectError::invalid_argument(err)
    }
}

pub fn event_from_request(
    branch: &str,
    commit: Option<&str>,
) -> Result<PullRequestEvent, ConnectError> {
    let provider: GitProviderKind = "github"
        .parse()
        .map_err(|e: String| ConnectError::invalid_argument(e))?;
    Ok(PullRequestEvent {
        provider,
        // The proto request only carries application_id/branch/commit;
        // the remaining webhook fields fall back to neutral defaults.
        owner: String::new(),
        repository: String::new(),
        number: "0".into(),
        action: "opened".into(),
        source_branch: branch.to_owned(),
        source_owner: None,
        source_repository: None,
        target_branch: branch.to_owned(),
        commit: commit.map(str::to_owned),
        author: None,
    })
}
