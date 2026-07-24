use std::sync::Arc;

use auto_route::controller;
use axum::{Json, http::StatusCode};
use sqlx::SqlitePool;

use crate::utils::docker::core::types::{NodeAvailability, SwarmRole};
use crate::{
    api::dto::swarm::{
        NodeActionDto, NodeAvailabilityDto, NodeDto, SwarmConnectionDto, SwarmInfoDto,
        SwarmTokensDto,
    },
    utils::{
        docker::DockerCli,
        exec::{CommandExecutor, ExecError, LocalExecutor, RemoteExecutor, SshAuth, SshHostKey},
        jwt::claim::Claims,
    },
};

type ApiError = (StatusCode, String);

pub struct SwarmController {
    db: Arc<SqlitePool>,
}

#[controller("/swarm")]
impl SwarmController {
    fn new(db: Arc<SqlitePool>) -> Self {
        Self { db }
    }

    #[post("/info")]
    async fn info(
        &self,
        _claims: Claims,
        Json(body): Json<SwarmConnectionDto>,
    ) -> Result<Json<SwarmInfoDto>, ApiError> {
        let docker = self.docker(body.server_id).await?;
        let swarm = docker.swarm().inspect().await.map_err(map_exec)?;

        Ok(Json(SwarmInfoDto {
            node_id: swarm.node_id,
            node_addr: swarm.node_addr,
            local_node_state: swarm.local_node_state,
            control_available: swarm.control_available,
            nodes: swarm.nodes as i64,
            managers: swarm.managers as i64,
        }))
    }

    #[post("/tokens")]
    async fn tokens(
        &self,
        _claims: Claims,
        Json(body): Json<SwarmConnectionDto>,
    ) -> Result<Json<SwarmTokensDto>, ApiError> {
        let docker = self.docker(body.server_id).await?;

        let worker = docker
            .swarm()
            .join_token()
            .get(SwarmRole::Worker)
            .await
            .map_err(map_exec)?;
        let manager = docker
            .swarm()
            .join_token()
            .get(SwarmRole::Manager)
            .await
            .map_err(map_exec)?;

        Ok(Json(SwarmTokensDto { worker, manager }))
    }

    #[post("/nodes")]
    async fn nodes(
        &self,
        _claims: Claims,
        Json(body): Json<SwarmConnectionDto>,
    ) -> Result<Json<Vec<NodeDto>>, ApiError> {
        let docker = self.docker(body.server_id).await?;
        let nodes = docker.nodes().list().run_json().await.map_err(map_exec)?;
        Ok(Json(nodes.into_iter().map(NodeDto::from).collect()))
    }

    #[post("/nodes/promote")]
    async fn promote(
        &self,
        _claims: Claims,
        Json(body): Json<NodeActionDto>,
    ) -> Result<StatusCode, ApiError> {
        let docker = self.docker(body.server_id).await?;
        docker
            .nodes()
            .promote(body.node_id)
            .run()
            .await
            .map_err(map_exec)?;
        Ok(StatusCode::NO_CONTENT)
    }

    #[post("/nodes/demote")]
    async fn demote(
        &self,
        _claims: Claims,
        Json(body): Json<NodeActionDto>,
    ) -> Result<StatusCode, ApiError> {
        let docker = self.docker(body.server_id).await?;
        docker
            .nodes()
            .demote(body.node_id)
            .run()
            .await
            .map_err(map_exec)?;
        Ok(StatusCode::NO_CONTENT)
    }

    #[post("/nodes/availability")]
    async fn set_availability(
        &self,
        _claims: Claims,
        Json(body): Json<NodeAvailabilityDto>,
    ) -> Result<StatusCode, ApiError> {
        let docker = self.docker(body.server_id).await?;
        let node_aval = NodeAvailability::try_from(body.availability.as_str())
            .map_err(|_| (StatusCode::BAD_REQUEST, "invalid availability value".into()))?;
        docker
            .nodes()
            .update(body.node_id)
            .availability(node_aval)
            .run()
            .await
            .map_err(map_exec)?;
        Ok(StatusCode::NO_CONTENT)
    }

    #[post("/nodes/remove")]
    async fn remove_node(
        &self,
        _claims: Claims,
        Json(body): Json<NodeActionDto>,
    ) -> Result<StatusCode, ApiError> {
        let docker = self.docker(body.server_id).await?;
        docker
            .nodes()
            .remove(body.node_id)
            .force()
            .run()
            .await
            .map_err(map_exec)?;
        Ok(StatusCode::NO_CONTENT)
    }

    #[post("/leave")]
    async fn leave(
        &self,
        _claims: Claims,
        Json(body): Json<SwarmConnectionDto>,
    ) -> Result<StatusCode, ApiError> {
        let docker = self.docker(body.server_id).await?;
        docker.swarm().leave().run().await.map_err(map_exec)?;
        Ok(StatusCode::NO_CONTENT)
    }

    async fn docker(&self, server_id: Option<i64>) -> Result<DockerCli, ApiError> {
        match server_id {
            None => Ok(DockerCli::from_executor(CommandExecutor::Local(
                LocalExecutor::new(),
            ))),
            Some(id) => {
                let executor = remote_executor_for(self.db.as_ref(), id).await?;
                Ok(DockerCli::from_remote_executor(executor))
            }
        }
    }
}

async fn remote_executor_for(db: &SqlitePool, server_id: i64) -> Result<RemoteExecutor, ApiError> {
    let row = sqlx::query_as::<_, (String, i64, String, String, String)>(
        r#"SELECT s.ip_address, s.port, s.username, k.private_key, k.public_key
           FROM servers s JOIN ssh_keys k ON k.id = s.ssh_key_id
           WHERE s.id = ?"#,
    )
    .bind(server_id)
    .fetch_one(db)
    .await
    .map_err(|_| (StatusCode::NOT_FOUND, "server or SSH key not found".into()))?;

    let port =
        u16::try_from(row.1).map_err(|_| (StatusCode::BAD_REQUEST, "invalid SSH port".into()))?;

    Ok(RemoteExecutor::new(
        row.0,
        port,
        row.2,
        SshAuth::key_pair(row.3, row.4),
        SshHostKey::InsecureAcceptAny,
    )
    .with_sudo())
}

fn map_exec(error: ExecError) -> ApiError {
    tracing::error!(error = %error, "swarm command failed");
    match error {
        ExecError::CommandFailed { .. } => (StatusCode::BAD_GATEWAY, error.to_string()),
        ExecError::Ssh(_) => (StatusCode::BAD_GATEWAY, error.to_string()),
        _ => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
    }
}
