use std::sync::Arc;

use auto_di::singleton;
use sha2::{Digest, Sha256};

use crate::{
    api::dto::backup::PanelBackupResponseDto, repository::BackupExecutionRepository,
    services::compose::management::ComposeTransferService, utils::paths::openoxide_paths,
};

pub struct ComposeConfigBackupService {
    transfer: Arc<ComposeTransferService>,
    executions: Arc<BackupExecutionRepository>,
}

#[singleton]
impl ComposeConfigBackupService {
    fn new(
        transfer: Arc<ComposeTransferService>,
        executions: Arc<BackupExecutionRepository>,
    ) -> Self {
        Self {
            transfer,
            executions,
        }
    }

    pub async fn create(
        &self,
        compose_id: i64,
        include_secrets: bool,
    ) -> sqlx::Result<PanelBackupResponseDto> {
        let bundle = self.transfer.export(compose_id, include_secrets).await?;
        let bytes = serde_json::to_vec_pretty(&bundle)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let output_dir = format!("{}/backups/compose", openoxide_paths().base);
        tokio::fs::create_dir_all(&output_dir)
            .await
            .map_err(sqlx::Error::Io)?;
        let path = format!(
            "{output_dir}/compose-{compose_id}-{}.json",
            chrono::Utc::now().format("%Y%m%d_%H%M%S")
        );
        let execution_id = self
            .executions
            .start(
                crate::services::backup::types::BackupKind::ComposeConfig,
                crate::services::backup::types::BackupOperation::Backup,
                Some(compose_id),
                Some(&path),
            )
            .await?;
        if let Err(error) = tokio::fs::write(&path, &bytes).await {
            self.executions
                .fail(execution_id, &error.to_string())
                .await?;
            return Err(sqlx::Error::Io(error));
        }
        let checksum = format!("{:x}", Sha256::digest(&bytes));
        let size_bytes = bytes.len() as i64;
        self.executions
            .succeed(execution_id, Some(&checksum), Some(size_bytes))
            .await?;
        Ok(PanelBackupResponseDto {
            path,
            checksum_sha256: checksum,
            size_bytes,
            created_at: chrono::Utc::now().timestamp(),
        })
    }
}
