use std::sync::Arc;

use auto_di::singleton;
use serde::Serialize;

use crate::{
    db::queries::FlashQueries,
    flash_gen::{models::ServerManagement, server_infra_extra::SrvInfManagementUpdateParams},
    utils::{
        exec::{CommandExecutor, ExecOutput},
        os::OsCli,
        setup::{ServerSetup, SetupConfig},
    },
};

pub struct ServerLifecycleService {
    queries: Arc<FlashQueries>,
}

#[singleton]
impl ServerLifecycleService {
    fn new(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }

    pub async fn auto_repair(&self, server_id: i64) -> sqlx::Result<ServerActionResultDto> {
        let executor = self.executor(server_id).await?;
        let outcome = ServerSetup::new_remote(remote(&executor)?, SetupConfig::default())
            .setup_all_oneshot(false)
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        Ok(ServerActionResultDto {
            success: true,
            stdout: format!(
                "repaired steps: {}",
                outcome
                    .completed
                    .iter()
                    .map(|step| format!("{step:?}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            stderr: String::new(),
        })
    }

    pub async fn configure_gpu(&self, server_id: i64) -> sqlx::Result<ServerActionResultDto> {
        let executor = self.executor(server_id).await?;
        let os = OsCli::new(&executor);
        os.gpu()
            .nvidia()
            .query([
                crate::utils::os::gpu::NvidiaQueryField::Name,
                crate::utils::os::gpu::NvidiaQueryField::DriverVersion,
            ])
            .format(crate::utils::os::gpu::NvidiaQueryFormat::Csv)
            .without_header()
            .run()
            .await
            .map_err(|error| {
                sqlx::Error::Protocol(format!("NVIDIA GPU/driver unavailable: {error}"))
            })?;
        let output = os
            .gpu()
            .nvidia()
            .configure()
            .runtime(crate::utils::os::gpu::ContainerRuntime::Docker)
            .run()
            .await
            .map_err(|error| {
                sqlx::Error::Protocol(format!("nvidia-container-toolkit unavailable: {error}"))
            })?;
        os.service("docker")
            .restart()
            .run()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let current = self.get_or_create(server_id).await?;
        self.queries
            .srv_inf_management_update(&SrvInfManagementUpdateParams {
                server_id,
                maintenance_mode: current.maintenance_mode,
                maintenance_message: current.maintenance_message.unwrap_or_default(),
                labels: current.labels,
                cleanup_policy: current.cleanup_policy,
                gpu_enabled: 1,
            })
            .await?;
        result(output)
    }

    pub async fn upgrade(&self, server_id: i64) -> sqlx::Result<ServerActionResultDto> {
        let executor = self.executor(server_id).await?;
        let update = OsCli::new(&executor)
            .package_api()
            .update_index()
            .run()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let upgrade = OsCli::new(&executor)
            .package_api()
            .upgrade_all()
            .run()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        Ok(ServerActionResultDto {
            success: true,
            stdout: format!("{}\n{}", update.stdout, upgrade.stdout),
            stderr: format!("{}\n{}", update.stderr, upgrade.stderr),
        })
    }

    pub async fn backup(&self, server_id: i64) -> sqlx::Result<ServerBackupDto> {
        let executor = self.executor(server_id).await?;
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let path = format!("/tmp/openoxide-server-{server_id}-{timestamp}.tar.gz");
        OsCli::new(&executor)
            .archive(&path)
            .tar()
            .create()
            .ignore_failed_reads()
            .entry("/etc/openoxide")
            .entry("/etc/docker")
            .run()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        Ok(ServerBackupDto {
            remote_path: path,
            created_at: chrono::Utc::now().timestamp(),
        })
    }

    pub async fn diagnostics(&self, server_id: i64) -> sqlx::Result<ServerActionResultDto> {
        let executor = self.executor(server_id).await?;
        let output = OsCli::new(&executor)
            .diagnostics()
            .server()
            .run()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        Ok(ServerActionResultDto {
            success: output.stderr.is_empty(),
            stdout: output.stdout,
            stderr: output.stderr,
        })
    }

    async fn get_or_create(&self, server_id: i64) -> sqlx::Result<ServerManagement> {
        self.queries
            .server_management_get_or_create1(server_id)
            .await?;
        self.queries
            .server_management_get_or_create2(server_id)
            .await
    }

    async fn executor(&self, server_id: i64) -> sqlx::Result<CommandExecutor> {
        self.queries.servers_get_by_id(server_id).await?;
        crate::services::compute::compose::remote::agent_executor(server_id)
            .await
            .map_err(sqlx::Error::Protocol)
    }
}

fn result(output: ExecOutput) -> sqlx::Result<ServerActionResultDto> {
    Ok(ServerActionResultDto {
        success: output.success(),
        stdout: output.stdout,
        stderr: output.stderr,
    })
}

// ── JSON payload contract (moved from the retired api::dto layer) ──────

#[derive(Debug, Clone, Serialize)]
pub struct ServerActionResultDto {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ServerBackupDto {
    pub remote_path: String,
    pub created_at: i64,
}

fn remote(executor: &CommandExecutor) -> sqlx::Result<crate::utils::exec::RemoteExecutor> {
    match executor {
        CommandExecutor::Remote(value) => Ok(*value.clone()),
        CommandExecutor::Local(_) => Err(sqlx::Error::Protocol("remote server required".into())),
        CommandExecutor::Agent(_) => {
            Err(sqlx::Error::Protocol("remote SSH executor required".into()))
        }
    }
}
