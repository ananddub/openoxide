use std::sync::Arc;

use auto_di::singleton;
use sha2::{Digest, Sha256};

use crate::{
    db::queries::FlashQueries,
    flash_gen::backup_executions::{
        BackupExecutionsNextAttemptParams, BackupExecutionsStartParams, BackupExecutionsSucceedParams,
    },
    services::compose::management::ComposeTransferService, utils::paths::openoxide_paths,
};

use super::panel::PanelBackupResponseDto;

pub struct ComposeConfigBackupService {
    transfer: Arc<ComposeTransferService>,
    queries: Arc<FlashQueries>,
}

#[singleton]
impl ComposeConfigBackupService {
    fn new(transfer: Arc<ComposeTransferService>, queries: Arc<FlashQueries>) -> Self {
        Self { transfer, queries }
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
        let backup_kind = crate::services::backup::types::BackupKind::ComposeConfig;
        let operation = crate::services::backup::types::BackupOperation::Backup;
        let attempt = self
            .queries
            .backup_executions_next_attempt(&BackupExecutionsNextAttemptParams {
                backup_kind: backup_kind.as_str().to_string(),
                operation: operation.as_str().to_string(),
                backup_id: compose_id,
            })
            .await?
            .unwrap_or(1);
        let execution_id = self
            .queries
            .backup_executions_start(&BackupExecutionsStartParams {
                backup_kind: backup_kind.as_str().to_string(),
                operation: operation.as_str().to_string(),
                backup_id: compose_id,
                object_key: path.clone(),
                attempt,
            })
            .await?;
        if let Err(error) = tokio::fs::write(&path, &bytes).await {
            self.queries
                .backup_executions_fail(&error.to_string(), execution_id)
                .await?;
            return Err(sqlx::Error::Io(error));
        }
        let checksum = format!("{:x}", Sha256::digest(&bytes));
        let size_bytes = bytes.len() as i64;
        self.queries
            .backup_executions_succeed(&BackupExecutionsSucceedParams {
                sha256_checksum: checksum.clone(),
                size_bytes,
                id: execution_id,
            })
            .await?;
        Ok(PanelBackupResponseDto {
            path,
            checksum_sha256: checksum,
            size_bytes,
            created_at: chrono::Utc::now().timestamp(),
        })
    }
}
