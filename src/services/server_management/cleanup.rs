use std::sync::Arc;

use auto_di::singleton;

use crate::{
    api::dto::server_management::{
        ServerActionResultDto, ServerCleanupExecutionDto, ServerCleanupPolicyDto,
    },
    repository::{ServerManagementRepository, ServerRepository},
    utils::{docker::DockerCli, exec::CommandExecutor, os::OsCli},
};

pub struct ServerCleanupService {
    db: Arc<sqlx::SqlitePool>,
    servers: Arc<ServerRepository>,
    management: Arc<ServerManagementRepository>,
}

#[singleton]
impl ServerCleanupService {
    fn new(
        db: Arc<sqlx::SqlitePool>,
        servers: Arc<ServerRepository>,
        management: Arc<ServerManagementRepository>,
    ) -> Self {
        Self {
            db,
            servers,
            management,
        }
    }

    pub async fn run(&self, server_id: i64) -> sqlx::Result<ServerActionResultDto> {
        let executor = self.executor(server_id).await?;
        let config = self.management.get_or_create(server_id).await?;
        let policy: ServerCleanupPolicyDto = serde_json::from_str(&config.cleanup_policy)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let execution_id = self
            .management
            .start_cleanup(server_id, &config.cleanup_policy)
            .await?;
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        let docker = DockerCli::from_executor(executor.clone());
        let result = async {
            if policy.containers || policy.images || policy.networks || policy.volumes {
                let output = docker
                    .system_prune(policy.images, policy.volumes, &[])
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
        self.management
            .finish_cleanup(execution_id, success, &stdout, &stderr)
            .await?;
        result?;
        Ok(ServerActionResultDto {
            success,
            stdout,
            stderr,
        })
    }

    pub async fn history(&self, server_id: i64) -> sqlx::Result<Vec<ServerCleanupExecutionDto>> {
        self.servers
            .get_by_id(server_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        self.management.list_cleanup(server_id).await.map(|items| {
            items
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
                .collect()
        })
    }

    async fn executor(&self, server_id: i64) -> sqlx::Result<CommandExecutor> {
        crate::services::compose::remote::remote_executor(self.db.as_ref(), server_id)
            .await
            .map(CommandExecutor::Remote)
            .map_err(sqlx::Error::Protocol)
    }
}
