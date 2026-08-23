use crate::api::proto::{ContainerStatePoint, ContainerStatesResponse};
use crate::docker::{api::DockerApi, types::ContainerSummary};
use std::collections::HashMap;
use tonic::Status;

pub struct StateService;

impl StateService {
    pub async fn current(docker: &DockerApi) -> Result<ContainerStatesResponse, Status> {
        let rows = docker
            .get_json::<Vec<ContainerSummary>>("/containers/json?all=true")
            .await
            .map_err(|error| Status::unavailable(error.to_string()))?;
        Ok(ContainerStatesResponse {
            containers: rows.into_iter().map(Self::container_state).collect(),
        })
    }

    pub async fn send_snapshot(
        docker: &DockerApi,
        tx: &tokio::sync::mpsc::Sender<Result<ContainerStatesResponse, Status>>,
    ) -> Result<(), ()> {
        let mut containers = Self::current(docker)
            .await
            .map_err(
                |error| tracing::debug!(%error, "could not read containers after Docker event"),
            )?
            .containers;
        containers.sort_by(|a, b| a.container_id.cmp(&b.container_id));
        tx.send(Ok(ContainerStatesResponse { containers }))
            .await
            .map_err(|_| ())
    }

    pub fn container_state(row: ContainerSummary) -> ContainerStatePoint {
        ContainerStatePoint {
            container_id: row.id.short().to_string(),
            name: row.display_name().as_str().to_string(),
            state: row.state,
            status: row.status,
            application_id: Self::label_id(&row.labels, "com.openoxide.application-id"),
            compose_id: Self::label_id(&row.labels, "com.openoxide.compose-id"),
            compose_project: row
                .labels
                .get("com.docker.compose.project")
                .cloned()
                .unwrap_or_default(),
            compose_service: row
                .labels
                .get("com.docker.compose.service")
                .cloned()
                .unwrap_or_default(),
        }
    }

    pub fn label_id(labels: &HashMap<String, String>, key: &str) -> i64 {
        labels
            .get(key)
            .and_then(|value| value.parse().ok())
            .unwrap_or(0)
    }
}
