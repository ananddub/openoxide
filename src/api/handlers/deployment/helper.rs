use connectrpc::{ConnectError, ErrorCode};
use serde::Deserialize;

use crate::api::connect::generated::openoxide::api::v1::DeploymentEvent;
use crate::flash_gen::models::Deployments;
use crate::services::compute::engine::DockerLogOptions;
use crate::utils::builder::custom_type::{ActiveDeploySnapshot, IdType};

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub enum DockerLogStream {
    #[default]
    All,
    Stdout,
    Stderr,
}

pub fn map_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::RowNotFound => ConnectError::not_found("deployment not found"),
        other => {
            tracing::error!(error = %other, "deployment operation failed");
            ConnectError::new(ErrorCode::Internal, "deployment operation failed")
        }
    }
}

pub fn deployment_to_proto(dto: &Deployments) -> crate::api::connect::generated::openoxide::api::v1::Deployment {
    crate::api::connect::generated::openoxide::api::v1::Deployment {
        id: dto.id,
        resource_type: dto.title.clone(),
        resource_id: dto.service_id.unwrap_or(0),
        status: dto.status.clone(),
        error: dto.error_message.clone(),
        logs: if dto.log_path.is_empty() { None } else { Some(dto.log_path.clone()) },
        created_at: dto.created_at,
        updated_at: dto.last_state_at.unwrap_or(dto.created_at),
        user_id: dto.user_id,
        ..Default::default()
    }
}

pub fn active_to_deployment(snapshot: &ActiveDeploySnapshot) -> crate::api::connect::generated::openoxide::api::v1::Deployment {
    let (kind, id) = match snapshot.id {
        IdType::AppId(id) => ("application", id),
        IdType::ComposeId(id) => ("compose", id),
        IdType::DatabaseId(id) => ("database", id),
    };
    crate::api::connect::generated::openoxide::api::v1::Deployment {
        id,
        resource_type: kind.to_string(),
        resource_id: id,
        status: format!("{:?}", snapshot.state),
        error: None,
        logs: None,
        created_at: 0,
        updated_at: 0,
        ..Default::default()
    }
}

pub fn docker_log_options(
    tail: Option<i64>,
    timestamps: Option<bool>,
    follow: Option<bool>,
    since: Option<&str>,
    until: Option<&str>,
) -> DockerLogOptions {
    let (since, until) = (since.map(str::to_owned), until.map(str::to_owned));
    DockerLogOptions {
        tail: tail.unwrap_or(200).clamp(1, 100_000) as usize,
        timestamps: timestamps.unwrap_or(false),
        follow: follow.unwrap_or(true),
        since,
        until,
    }
}

pub fn empty_event_stream(
) -> impl futures::Stream<Item = Result<DeploymentEvent, ConnectError>> + Send + 'static {
    futures::stream::empty()
}

/// Adapts a docker CLI stream (stdout = data, stderr = failure) into
/// DeploymentEvents of the given kind. Each stdout chunk becomes one event
/// carrying the raw `{{json .}}` output.
pub fn docker_event_stream(
    receiver: tokio::sync::mpsc::Receiver<crate::utils::docker::DockerStreamEvent>,
    kind: &'static str,
) -> impl futures::Stream<Item = Result<DeploymentEvent, ConnectError>> + Send + 'static {
    use futures::StreamExt;
    tokio_stream::wrappers::ReceiverStream::new(receiver).map(|event| {
        let is_error = matches!(
            event,
            crate::utils::docker::DockerStreamEvent::Stderr(_)
        );
        let output = match event {
            crate::utils::docker::DockerStreamEvent::Stdout(bytes) => bytes,
            crate::utils::docker::DockerStreamEvent::Stderr(bytes) => bytes,
        };
        let payload = String::from_utf8_lossy(&output).into_owned();
        let error = is_error.then(|| payload.clone());
        Ok(DeploymentEvent {
            r#type: kind.to_owned(),
            timestamp: chrono::Utc::now().timestamp(),
            payload,
            error,
        })
    })
}

/// Adapts a deployment log line channel into log DeploymentEvents.
pub fn log_line_stream(
    receiver: tokio::sync::mpsc::Receiver<String>,
) -> impl futures::Stream<Item = Result<DeploymentEvent, ConnectError>> + Send + 'static {
    use futures::StreamExt;
    tokio_stream::wrappers::ReceiverStream::new(receiver).map(|line| {
        Ok(DeploymentEvent {
            r#type: "log".to_owned(),
            timestamp: chrono::Utc::now().timestamp(),
            payload: line,
            error: None,
        })
    })
}

pub fn sanitize_filename(value: &str) -> String {
    let sanitized: String = value
        .chars()
        .map(|character| match character {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' => character,
            _ => '_',
        })
        .collect();
    if sanitized.is_empty() {
        "container".into()
    } else {
        sanitized
    }
}

pub fn select_log_output(
    output: crate::utils::docker::DockerOutput,
    selector: DockerLogStream,
) -> Vec<u8> {
    match selector {
        DockerLogStream::All => {
            format!("{}{}", output.stdout, output.stderr).into_bytes()
        }
        DockerLogStream::Stdout => output.stdout.into_bytes(),
        DockerLogStream::Stderr => output.stderr.into_bytes(),
    }
}
