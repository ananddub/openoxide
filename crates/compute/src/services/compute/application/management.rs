use auto_di::resolve;
use sqlx::SqlitePool;

use crate::core::cache::CacheKey;
use crate::db::queries::FlashQueries;
use crate::flash_gen::deployments::{
    DeploymentsFinalizeWithResource1Params, DeploymentsFinalizeWithResource3Params,
};
use crate::flash_gen::services::ServicesMoveToEnvironmentInSameOrganizationParams;
use crate::services::infrastructure::server::ssh_credentials;
use crate::utils::builder::queue::BuilderQueue;
use crate::utils::exec::{RemoteExecutor, SshAuth, SshHostKey};

use super::ApplicationService;

#[derive(Debug, Clone, Copy)]
pub struct ApplicationCleanupResult {
    pub affected: u64,
}

#[derive(Debug, Clone)]
pub struct ApplicationForceKillResult {
    pub deployment_id: i64,
    pub result: crate::services::compute::engine::CancelDeploymentResult,
}

impl ApplicationService {
    pub async fn force_kill_deployment(&self, id: i64) -> sqlx::Result<ApplicationForceKillResult> {
        self.get_by_id(id).await?;
        let deployment = match self.queries.deployments_running_for_application(id).await {
            Ok(deployment) => deployment,
            Err(sqlx::Error::RowNotFound) => {
                return Err(sqlx::Error::Protocol(
                    "application has no running deployment".into(),
                ));
            }
            Err(error) => return Err(error),
        };
        let deployment_id = deployment.id;
        let mut result = self.deployment_service.cancel(deployment_id).await?;

        if matches!(
            result,
            crate::services::compute::engine::CancelDeploymentResult::NotActiveInThisProcess
        ) && let (Some(server_id), Some(pid_file)) =
            (deployment.server_id, deployment.pid.as_deref())
        {
            let executor = crate::services::compute::compose::remote::agent_executor(server_id)
                .await
                .map_err(sqlx::Error::Protocol)?;
            executor
                .kill_pid_file(pid_file)
                .await
                .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
            let error_message = "deployment force-killed";
            self.queries
                .deployments_finalize_with_resource1(&DeploymentsFinalizeWithResource1Params {
                    status: "CANCELLED".to_owned(),
                    state: "CANCELLED".to_owned(),
                    error_message: error_message.to_owned(),
                    id: deployment_id,
                })
                .await?;
            let _ =
                sqlx::query("UPDATE deployments SET logs = COALESCE(logs, '') || ? WHERE id = ?;")
                    .bind(format!("\n[ERROR] {error_message}\n"))
                    .bind(deployment_id)
                    .execute(&self.queries.pool)
                    .await;
            self.queries
                .deployments_finalize_with_resource3(&DeploymentsFinalizeWithResource3Params {
                    app_status: "ERROR".to_owned(),
                    last_error: error_message.to_owned(),
                    id,
                })
                .await?;
            self.queries
                .deployments_finalize_with_resource6(deployment_id)
                .await?;
            result = crate::services::compute::engine::CancelDeploymentResult::CancelRequested;
        }
        Ok(ApplicationForceKillResult {
            deployment_id,
            result,
        })
    }

    pub async fn move_to_environment(
        &self,
        id: i64,
        target_environment_id: i64,
    ) -> sqlx::Result<crate::services::compute::application::ApplicationRecord> {
        self.get_by_id(id).await?;
        if self
            .queries
            .deployments_has_running_deployment(&id.to_string())
            .await?
        {
            return Err(sqlx::Error::Protocol(
                "application deployment is already queued or running".into(),
            ));
        }

        self.queries
            .services_move_to_environment_in_same_organization(
                &ServicesMoveToEnvironmentInSameOrganizationParams {
                    environment_id: target_environment_id,
                    id,
                    id2: target_environment_id,
                },
            )
            .await?;
        self.cache.invalidate(&CacheKey::Application(id)).await;
        let record = self.get_by_id(id).await?;
        if record.environment_id != target_environment_id {
            return Err(sqlx::Error::Protocol(
                "target environment not found or belongs to another organization".into(),
            ));
        }
        Ok(record)
    }

    pub async fn rotate_webhook_token(&self, id: i64) -> sqlx::Result<String> {
        self.get_by_id(id).await?;
        let mut bytes = [0_u8; 32];
        getrandom::fill(&mut bytes).map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let token: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
        // Old repository treated rows_affected == 1 as success; the generator
        // cannot surface affected counts, so this stays raw SQL.
        let affected = sqlx::query(
            "UPDATE services SET refresh_token = ?, updated_at = unixepoch() WHERE id = ?;",
        )
        .bind(&token)
        .bind(id)
        .execute(&self.queries.pool)
        .await?
        .rows_affected();
        if affected != 1 {
            return Err(sqlx::Error::RowNotFound);
        }
        self.cache.invalidate(&CacheKey::Application(id)).await;
        Ok(token)
    }

    pub async fn clear_deployment_history(
        &self,
        id: i64,
    ) -> sqlx::Result<ApplicationCleanupResult> {
        self.get_by_id(id).await?;
        let deployment_ids = self
            .queries
            .deployments_list_finished_ids_for_application(id)
            .await?;
        // The generated delete returns (); the affected count the caller
        // expects requires the raw statement.
        let affected = sqlx::query(
            "DELETE FROM deployments WHERE service_id = ? AND status NOT IN ('QUEUED', 'RUNNING');",
        )
        .bind(id)
        .execute(&self.queries.pool)
        .await?
        .rows_affected();

        for deployment_id in deployment_ids {
            let path = crate::utils::paths::openoxide_paths().deployment_log_file(deployment_id);
            match tokio::fs::remove_file(&path).await {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => {
                    tracing::warn!(deployment_id, %error, "could not delete deployment log")
                }
            }
        }
        Ok(ApplicationCleanupResult { affected })
    }

    pub async fn cleanup_deployment_queue(
        &self,
        id: i64,
    ) -> sqlx::Result<ApplicationCleanupResult> {
        self.get_by_id(id).await?;
        if let Ok(queue) = resolve::<BuilderQueue>().await {
            let _ = queue.cancel_queued_application(id).await;
        }
        // Same affected-count requirement as clear_deployment_history.
        let affected = sqlx::query(
            "UPDATE deployments SET status = 'CANCELLED', state = 'CANCELLED', finished_at = unixepoch(), last_state_at = unixepoch() WHERE service_id = ? AND status = 'QUEUED';",
        )
        .bind(id)
        .execute(&self.queries.pool)
        .await?
        .rows_affected();
        Ok(ApplicationCleanupResult { affected })
    }
}

pub(crate) async fn remote_executor(
    _db: &SqlitePool,
    server_id: i64,
) -> Result<RemoteExecutor, String> {
    let queries = resolve::<FlashQueries>()
        .await
        .map_err(|error| format!("could not resolve database queries: {error}"))?;
    let (host, port, username, private_key, public_key) = ssh_credentials(&queries, server_id)
        .await
        .map_err(|error| format!("could not load SSH credentials: {error}"))?
        .ok_or_else(|| "Server not found".to_string())?;

    let port = u16::try_from(port).map_err(|_| "SSH port must be between 0 and 65535")?;
    let host_key = host_key_for_server(&queries, server_id).await;
    let mut executor = RemoteExecutor::new(
        host,
        port,
        username.clone(),
        SshAuth::key_pair(private_key, public_key),
        host_key,
    );
    if username != "root" {
        executor = executor.with_sudo();
    }
    Ok(executor)
}

/// Resolve the host-key verification policy for a server from its stored pin.
/// Falls back to accepting any key (with a warning) only when no fingerprint
/// was captured during setup, preserving pre-migration behaviour.
pub(crate) async fn host_key_for_server(queries: &FlashQueries, server_id: i64) -> SshHostKey {
    match queries.servers_get_host_key_fingerprint(server_id).await {
        Ok(fp) if !fp.trim().is_empty() => SshHostKey::PinnedSha256(fp.trim().to_owned()),
        _ => {
            tracing::warn!(
                server_id,
                "deployment SSH host key verification is disabled because no fingerprint is stored for this server"
            );
            SshHostKey::InsecureAcceptAny
        }
    }
}
