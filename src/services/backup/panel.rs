use std::{path::Path, sync::Arc};

use auto_di::singleton;
use sha2::{Digest, Sha256};
use sqlx::SqlitePool;

use crate::{
    core::config::Config,
    db::queries::FlashQueries,
    flash_gen::backup_executions::{
        BackupExecutionsNextAttemptParams, BackupExecutionsStartParams, BackupExecutionsSucceedParams,
    },
    utils::{
        exec::{CommandExecutor, LocalExecutor},
        os::OsCli,
        paths::openoxide_paths,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct PanelBackupResponseDto {
    pub path: String,
    pub checksum_sha256: String,
    pub size_bytes: i64,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StagePanelRestoreDto {
    pub restore_id: String,
    pub checksum_sha256: String,
    pub restart_required: bool,
    pub pending_marker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelRestoreStatusDto {
    pub restore_id: String,
    pub status: String,
    pub message: String,
    pub updated_at: i64,
}

pub struct PanelBackupService {
    db: Arc<SqlitePool>,
    config: Arc<Config>,
    queries: Arc<FlashQueries>,
}

#[singleton]
impl PanelBackupService {
    fn new(db: Arc<SqlitePool>, config: Arc<Config>, queries: Arc<FlashQueries>) -> Self {
        Self { db, config, queries }
    }

    pub async fn create(&self) -> sqlx::Result<PanelBackupResponseDto> {
        let paths = openoxide_paths();
        let output_dir = format!("{}/backups/panel", paths.base);
        tokio::fs::create_dir_all(&output_dir)
            .await
            .map_err(io_error)?;
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let archive = format!("{output_dir}/openoxide-panel-{timestamp}.tar.gz");
        let staging = tempfile::tempdir().map_err(io_error)?;
        let snapshot = staging.path().join("db.sqlite3");
        let snapshot_string = snapshot.to_string_lossy().into_owned();
        let backup_kind = crate::services::backup::types::BackupKind::Panel;
        let operation = crate::services::backup::types::BackupOperation::Backup;
        let attempt = self
            .queries
            .backup_executions_next_attempt(&BackupExecutionsNextAttemptParams {
                backup_kind: backup_kind.as_str().to_string(),
                operation: operation.as_str().to_string(),
                backup_id: 0,
            })
            .await?
            .unwrap_or(1);
        let execution_id = self
            .queries
            .backup_executions_start(&BackupExecutionsStartParams {
                backup_kind: backup_kind.as_str().to_string(),
                operation: operation.as_str().to_string(),
                backup_id: 0,
                object_key: archive.clone(),
                attempt,
            })
            .await?;

        let result = async {
            sqlx::query("VACUUM INTO ?")
                .bind(&snapshot_string)
                .execute(self.db.as_ref())
                .await?;
            let manifest = serde_json::json!({
                "format": "openoxide.panel.v1",
                "created_at": chrono::Utc::now().timestamp(),
                "database_url": redact_database_url(&self.config.database_url),
            });
            tokio::fs::write(
                staging.path().join("manifest.json"),
                serde_json::to_vec_pretty(&manifest)
                    .map_err(|error| sqlx::Error::Protocol(error.to_string()))?,
            )
            .await
            .map_err(io_error)?;

            let executor = CommandExecutor::Local(LocalExecutor::new());
            let staging_path = staging.path().to_string_lossy().into_owned();
            let traefik = format!("{}/traefik", paths.base);
            let os = OsCli::new(&executor);
            let mut archive_builder = os
                .archive(&archive)
                .tar()
                .create()
                .entry_from(&staging_path, "db.sqlite3")
                .entry_from(&staging_path, "manifest.json");
            if Path::new(&traefik).exists() {
                archive_builder = archive_builder.entry_from(&paths.base, "traefik");
            }
            archive_builder
                .run()
                .await
                .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
            let verification = os
                .archive(&archive)
                .tar()
                .list()
                .run()
                .await
                .map_err(|error| {
                    sqlx::Error::Protocol(format!("panel backup verification failed: {error}"))
                })?;
            if !verification
                .stdout
                .lines()
                .any(|entry| entry == "db.sqlite3")
                || !verification
                    .stdout
                    .lines()
                    .any(|entry| entry == "manifest.json")
            {
                return Err(sqlx::Error::Protocol(
                    "panel backup verification failed: required files are missing".into(),
                ));
            }
            let bytes = tokio::fs::read(&archive).await.map_err(io_error)?;
            let checksum = format!("{:x}", Sha256::digest(&bytes));
            Ok::<_, sqlx::Error>((checksum, bytes.len() as i64))
        }
        .await;

        match result {
            Ok((checksum, size_bytes)) => {
                self.queries
                    .backup_executions_succeed(&BackupExecutionsSucceedParams {
                        sha256_checksum: checksum.clone(),
                        size_bytes,
                        id: execution_id,
                    })
                    .await?;
                Ok(PanelBackupResponseDto {
                    path: archive,
                    checksum_sha256: checksum,
                    size_bytes,
                    created_at: chrono::Utc::now().timestamp(),
                })
            }
            Err(error) => {
                self.queries
                    .backup_executions_fail(&error.to_string(), execution_id)
                    .await?;
                Err(error)
            }
        }
    }

    pub async fn stage_restore(
        &self,
        archive: &str,
        checksum_sha256: Option<&str>,
    ) -> sqlx::Result<StagePanelRestoreDto> {
        validate_local_archive_path(archive)?;
        let bytes = tokio::fs::read(archive).await.map_err(io_error)?;
        let checksum = format!("{:x}", Sha256::digest(&bytes));
        if let Some(expected) = checksum_sha256
            && !checksum.eq_ignore_ascii_case(expected.trim())
        {
            return Err(sqlx::Error::Protocol("backup checksum mismatch".into()));
        }
        let executor = CommandExecutor::Local(LocalExecutor::new());
        let os = OsCli::new(&executor);
        let listing = os
            .archive(archive)
            .tar()
            .list()
            .run()
            .await
            .map_err(|error| sqlx::Error::Protocol(format!("invalid panel archive: {error}")))?;
        for entry in listing.stdout.lines() {
            let path = Path::new(entry);
            if path.is_absolute()
                || path
                    .components()
                    .any(|part| matches!(part, std::path::Component::ParentDir))
            {
                return Err(sqlx::Error::Protocol(
                    "panel archive contains an unsafe path".into(),
                ));
            }
        }
        if !listing.stdout.lines().any(|entry| entry == "db.sqlite3") {
            return Err(sqlx::Error::Protocol(
                "panel archive does not contain db.sqlite3".into(),
            ));
        }
        let paths = openoxide_paths();
        let restore_id = uuid::Uuid::new_v4().simple().to_string();
        let staging = format!("{}/backups/restore-staging/{restore_id}", paths.base);
        tokio::fs::create_dir_all(&staging)
            .await
            .map_err(io_error)?;
        os.archive(archive)
            .tar()
            .extract_to(&staging)
            .run()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let marker = format!("{}/backups/panel-restore.pending.json", paths.base);
        let marker_body = serde_json::json!({
            "restore_id": restore_id.clone(),
            "staging": staging,
            "checksum_sha256": checksum,
            "created_at": chrono::Utc::now().timestamp(),
        });
        tokio::fs::write(
            &marker,
            serde_json::to_vec_pretty(&marker_body)
                .map_err(|error| sqlx::Error::Protocol(error.to_string()))?,
        )
        .await
        .map_err(io_error)?;
        Ok(StagePanelRestoreDto {
            restore_id,
            checksum_sha256: checksum,
            restart_required: true,
            pending_marker: marker,
        })
    }

    pub async fn restore_status(&self, restore_id: &str) -> sqlx::Result<PanelRestoreStatusDto> {
        if restore_id.is_empty() || !restore_id.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(sqlx::Error::Protocol("invalid restore id".into()));
        }
        let paths = openoxide_paths();
        let pending = format!("{}/backups/panel-restore.pending.json", paths.base);
        if let Ok(bytes) = tokio::fs::read(&pending).await
            && let Ok(value) = serde_json::from_slice::<serde_json::Value>(&bytes)
            && value.get("restore_id").and_then(|v| v.as_str()) == Some(restore_id)
        {
            return Ok(PanelRestoreStatusDto {
                restore_id: restore_id.into(),
                status: "PENDING_RESTART".into(),
                message: "restore is staged and will be applied during panel restart".into(),
                updated_at: value
                    .get("created_at")
                    .and_then(|v| v.as_i64())
                    .unwrap_or_default(),
            });
        }
        let completed = format!("{}/backups/restore-history/{restore_id}.json", paths.base);
        let bytes = tokio::fs::read(completed).await.map_err(io_error)?;
        serde_json::from_slice(&bytes).map_err(|error| sqlx::Error::Protocol(error.to_string()))
    }

    pub async fn stage_rollback(
        &self,
        recovery_database: &str,
    ) -> sqlx::Result<StagePanelRestoreDto> {
        let paths = openoxide_paths();
        let database = Path::new(recovery_database);
        let allowed_parent = Path::new(&paths.base);
        if !database.is_absolute()
            || !database.starts_with(allowed_parent)
            || !database
                .file_name()
                .is_some_and(|name| name.to_string_lossy().contains(".pre-restore-"))
        {
            return Err(sqlx::Error::Protocol(
                "recovery database must be a OpenOxide pre-restore snapshot".into(),
            ));
        }
        let bytes = tokio::fs::read(database).await.map_err(io_error)?;
        let restore_id = uuid::Uuid::new_v4().simple().to_string();
        let staging = format!("{}/backups/restore-staging/{restore_id}", paths.base);
        tokio::fs::create_dir_all(&staging)
            .await
            .map_err(io_error)?;
        tokio::fs::write(format!("{staging}/db.sqlite3"), &bytes)
            .await
            .map_err(io_error)?;
        let checksum = format!("{:x}", Sha256::digest(&bytes));
        let marker = format!("{}/backups/panel-restore.pending.json", paths.base);
        tokio::fs::write(&marker, serde_json::to_vec_pretty(&serde_json::json!({ "restore_id": restore_id.clone(), "staging": staging, "checksum_sha256": checksum, "created_at": chrono::Utc::now().timestamp(), "kind": "ROLLBACK" })).map_err(|error| sqlx::Error::Protocol(error.to_string()))?).await.map_err(io_error)?;
        Ok(StagePanelRestoreDto {
            restore_id,
            checksum_sha256: checksum,
            restart_required: true,
            pending_marker: marker,
        })
    }
}

fn validate_local_archive_path(value: &str) -> sqlx::Result<()> {
    let path = Path::new(value);
    let backup_root = format!("{}/backups", openoxide_paths().base);
    if !path.is_absolute() || !path.starts_with(&backup_root) || !value.ends_with(".tar.gz") {
        return Err(sqlx::Error::Protocol(
            "archive must be a .tar.gz file inside the OpenOxide backup directory".into(),
        ));
    }
    Ok(())
}

fn redact_database_url(value: &str) -> String {
    value.split('?').next().unwrap_or(value).to_string()
}

fn io_error(error: std::io::Error) -> sqlx::Error {
    sqlx::Error::Io(error)
}
