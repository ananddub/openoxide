use std::sync::Arc;

use auto_di::singleton;

use crate::{
    api::dto::backup::{BackupFileDto, BackupIntegrityDto, RetentionPreviewDto},
    repository::DestinationRepository,
    utils::{
        backup::database::S3Destination,
        exec::{CommandExecutor, LocalExecutor},
        rclone::{RcloneBuilder, RcloneCommand, RcloneListFormat, RcloneSeparator},
    },
};

pub struct BackupFileService {
    destinations: Arc<DestinationRepository>,
}

#[singleton]
impl BackupFileService {
    fn new(destinations: Arc<DestinationRepository>) -> Self {
        Self { destinations }
    }

    pub async fn list(
        &self,
        destination_id: i64,
        prefix: &str,
    ) -> sqlx::Result<Vec<BackupFileDto>> {
        validate_object_path(prefix)?;
        let destination = self.destination(destination_id).await?;
        let output = RcloneBuilder::new(RcloneCommand::Lsf)
            .source(destination.to_rclone_target(prefix))
            .recursive()
            .files_only()
            .list_format(RcloneListFormat::PathSizeModified)
            .separator(RcloneSeparator::Tab)
            .execute(&CommandExecutor::Local(LocalExecutor::new()))
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        Ok(output.stdout.lines().filter_map(parse_lsf_line).collect())
    }

    pub async fn download(&self, destination_id: i64, object_key: &str) -> sqlx::Result<Vec<u8>> {
        validate_object_path(object_key)?;
        let destination = self.destination(destination_id).await?;
        let temp = tempfile::NamedTempFile::new().map_err(sqlx::Error::Io)?;
        let local_path = temp.path().to_string_lossy().into_owned();
        RcloneBuilder::new(RcloneCommand::Copyto)
            .source(destination.to_rclone_target(object_key))
            .destination(crate::utils::rclone::RcloneTarget::Local {
                path: local_path.clone(),
            })
            .execute(&CommandExecutor::Local(LocalExecutor::new()))
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        tokio::fs::read(local_path).await.map_err(sqlx::Error::Io)
    }

    pub async fn retention_preview(
        &self,
        destination_id: i64,
        prefix: &str,
        keep_latest: i64,
    ) -> sqlx::Result<RetentionPreviewDto> {
        if keep_latest < 1 {
            return Err(sqlx::Error::Protocol(
                "keep_latest must be at least 1".into(),
            ));
        }
        let mut files = self.list(destination_id, prefix).await?;
        files.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));
        let delete = files.split_off((keep_latest as usize).min(files.len()));
        Ok(RetentionPreviewDto {
            keep: files,
            delete,
        })
    }

    pub async fn verify(
        &self,
        destination_id: i64,
        object_key: &str,
        expected_sha256: &str,
    ) -> sqlx::Result<BackupIntegrityDto> {
        use sha2::{Digest, Sha256};
        let bytes = self.download(destination_id, object_key).await?;
        let actual_sha256 = format!("{:x}", Sha256::digest(&bytes));
        Ok(BackupIntegrityDto {
            valid: actual_sha256.eq_ignore_ascii_case(expected_sha256.trim()),
            expected_sha256: expected_sha256.trim().to_ascii_lowercase(),
            actual_sha256,
            size_bytes: bytes.len() as i64,
        })
    }

    async fn destination(&self, id: i64) -> sqlx::Result<S3Destination> {
        let value = self
            .destinations
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        Ok(S3Destination {
            access_key: value.access_key,
            secret_key: value.secret_access_key,
            bucket: value.bucket,
            region: value.region,
            endpoint: value.endpoint,
            provider: Some(value.provider),
        })
    }
}

fn parse_lsf_line(line: &str) -> Option<BackupFileDto> {
    let mut fields = line.split('\t');
    let path = fields.next()?.trim().to_string();
    let size_bytes = fields.next()?.trim().parse().ok()?;
    let modified_at = fields.next().unwrap_or_default().trim().to_string();
    Some(BackupFileDto {
        name: path.rsplit('/').next().unwrap_or(&path).to_string(),
        path,
        size_bytes,
        modified_at,
    })
}

fn validate_object_path(value: &str) -> sqlx::Result<()> {
    if value.starts_with('/') || value.split('/').any(|part| part == "..") {
        return Err(sqlx::Error::Protocol("unsafe backup object path".into()));
    }
    Ok(())
}
