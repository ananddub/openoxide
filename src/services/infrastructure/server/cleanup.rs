use std::sync::Arc;

use auto_di::singleton;
use serde::Serialize;

use crate::{
    db::queries::FlashQueries,
    flash_gen::server_infra_extra::SrvInfFinishCleanupParams,
    utils::{docker::DockerCli, exec::CommandExecutor, os::OsCli},
};

use super::{lifecycle::ServerActionResultDto, management::ServerCleanupPolicyDto};

pub struct ServerCleanupService {
    queries: Arc<FlashQueries>,
}

#[singleton]
impl ServerCleanupService {
    fn new(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }

    pub async fn run(&self, server_id: i64) -> sqlx::Result<ServerActionResultDto> {
        let executor = self.executor(server_id).await?;
        let config = self.get_or_create(server_id).await?;
        let policy: ServerCleanupPolicyDto = serde_json::from_str(&config.cleanup_policy)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let execution_id = self
            .queries
            .srv_inf_start_cleanup(server_id, &config.cleanup_policy)
            .await?;
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        let docker = DockerCli::from_executor(executor.clone());
        let result = async {
            if policy.containers || policy.images || policy.networks || policy.volumes {
                let output = docker
                    .system()
                    .prune()
                    .all(policy.images)
                    .volumes(policy.volumes)
                    .run()
                    .await
                    .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
                stdout.push(output.stdout);
                stderr.push(output.stderr);
            }
            if policy.packages {
                let output = OsCli::new(&executor)
                    .package_api()
                    .clean()
                    .run()
                    .await
                    .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
                stdout.push(output.stdout);
                stderr.push(output.stderr);
            }
            Ok::<(), sqlx::Error>(())
        }
        .await;
        let success = result.is_ok();
        let stdout = stdout.join("\n");
        let mut stderr = stderr.join("\n");
        if let Err(error) = &result {
            stderr.push_str(&format!("\n{error}"));
        }
        self.queries
            .srv_inf_finish_cleanup(&SrvInfFinishCleanupParams {
                status: if success { "SUCCEEDED" } else { "FAILED" }.into(),
                stdout: stdout.clone(),
                stderr: stderr.clone(),
                id: execution_id,
            })
            .await?;
        result?;
        Ok(ServerActionResultDto {
            success,
            stdout,
            stderr,
        })
    }

    pub async fn history(&self, server_id: i64) -> sqlx::Result<Vec<ServerCleanupExecutionDto>> {
        self.queries.servers_get_by_id(server_id).await?;
        Ok(self
            .queries
            .server_management_list_cleanup(server_id)
            .await?
            .into_iter()
            .map(|value| ServerCleanupExecutionDto {
                id: value.id,
                server_id: value.server_id,
                status: value.status,
                policy: value.policy,
                stdout: value.stdout,
                stderr: value.stderr,
                started_at: value.started_at,
                finished_at: value.finished_at,
            })
            .collect())
    }

    async fn get_or_create(
        &self,
        server_id: i64,
    ) -> sqlx::Result<crate::flash_gen::models::ServerManagement> {
        self.queries.server_management_get_or_create1(server_id).await?;
        self.queries
            .server_management_get_or_create2(server_id)
            .await
    }

    async fn executor(&self, server_id: i64) -> sqlx::Result<CommandExecutor> {
        crate::services::compute::compose::remote::agent_executor(server_id)
            .await
            .map_err(sqlx::Error::Protocol)
    }
}

// ── JSON payload contract (moved from the retired api::dto layer) ──────

#[derive(Debug, Clone, Serialize)]
pub struct ServerCleanupExecutionDto {
    pub id: i64,
    pub server_id: i64,
    pub status: String,
    pub policy: String,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub started_at: i64,
    pub finished_at: Option<i64>,
}
