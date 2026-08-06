use std::{path::Path, sync::Arc};

use auto_di::singleton;
use sha2::{Digest, Sha256};
use sqlx::SqlitePool;

use crate::{
    api::dto::backup::{PanelBackupResponseDto, StagePanelRestoreDto},
    core::config::Config,
    repository::BackupExecutionRepository,
    utils::{
        exec::{CommandExecutor, LocalExecutor},
        os::OsCli,
        paths::rustploy_paths,
    },
};

pub struct PanelBackupService {
    db: Arc<SqlitePool>,
    config: Arc<Config>,
    executions: Arc<BackupExecutionRepository>,
}

#[singleton]
impl PanelBackupService {
    fn new(
        db: Arc<SqlitePool>,
        config: Arc<Config>,
        executions: Arc<BackupExecutionRepository>,
    ) -> Self {
        Self {
            db,
            config,
            executions,
        }
    }

    pub async fn create(&self) -> sqlx::Result<PanelBackupResponseDto> {
        let paths = rustploy_paths();
        let output_dir = format!("{}/backups/panel", paths.base);
        tokio::fs::create_dir_all(&output_dir)
            .await
            .map_err(io_error)?;
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let archive = format!("{output_dir}/rustploy-panel-{timestamp}.tar.gz");
        let staging = tempfile::tempdir().map_err(io_error)?;
        let snapshot = staging.path().join("db.sqlite3");
        let snapshot_string = snapshot.to_string_lossy().into_owned();
        let execution_id = self
            .executions
            .start(
                crate::services::backup::types::BackupKind::Panel,
                crate::services::backup::types::BackupOperation::Backup,
                None,
                Some(&archive),
            )
            .await?;

        let result = async {
            sqlx::query("VACUUM INTO ?")
                .bind(&snapshot_string)
                .execute(self.db.as_ref())
                .await?;
            let manifest = serde_json::json!({
                "format": "rustploy.panel.v1",
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
                self.executions
                    .succeed(execution_id, Some(&checksum), Some(size_bytes))
                    .await?;
                Ok(PanelBackupResponseDto {
                    path: archive,
                    checksum_sha256: checksum,
                    size_bytes,
                    created_at: chrono::Utc::now().timestamp(),
                })
            }
            Err(error) => {
                self.executions
                    .fail(execution_id, &error.to_string())
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
        let paths = rustploy_paths();
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
}

fn validate_local_archive_path(value: &str) -> sqlx::Result<()> {
    let path = Path::new(value);
    let backup_root = format!("{}/backups", rustploy_paths().base);
    if !path.is_absolute() || !path.starts_with(&backup_root) || !value.ends_with(".tar.gz") {
        return Err(sqlx::Error::Protocol(
            "archive must be a .tar.gz file inside the Rustploy backup directory".into(),
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
