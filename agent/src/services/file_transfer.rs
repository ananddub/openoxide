use crate::{api::proto, core::config::Config};
use auto_di::singleton;
use sha2::{Digest, Sha256};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;

#[derive(Clone)]
pub struct FileTransferService {
    config: Arc<Config>,
}

#[singleton]
impl FileTransferService {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn agent_info(&self) -> proto::AgentInfoResponse {
        proto::AgentInfoResponse {
            protocol_version: "1".into(),
            agent_version: env!("CARGO_PKG_VERSION").into(),
            capabilities: vec!["file_upload".into()],
            max_upload_bytes: self.config.max_upload_bytes,
            upload_roots: self.config.upload_roots.clone(),
        }
    }

    pub async fn upload(
        &self,
        mut stream: tonic::Streaming<proto::FileUploadChunk>,
    ) -> Result<proto::FileUploadResult, tonic::Status> {
        let first = tokio::time::timeout(
            std::time::Duration::from_secs(self.config.upload_timeout_seconds),
            stream.next(),
        )
        .await
        .map_err(|_| tonic::Status::deadline_exceeded("upload timed out"))?
        .ok_or_else(|| tonic::Status::invalid_argument("upload stream is empty"))?
        .map_err(|e| tonic::Status::invalid_argument(format!("read upload: {e}")))?;

        if first.server_id != self.config.server_id {
            return Err(tonic::Status::not_found(
                "upload server_id does not belong to this agent",
            ));
        }
        let destination = validate_destination(&first.destination, &self.config.upload_roots)?;
        if first.total_size == 0 || first.total_size > self.config.max_upload_bytes {
            return Err(tonic::Status::invalid_argument("invalid upload size"));
        }
        let expected = first.checksum_sha256.trim().to_ascii_lowercase();
        if expected.len() != 64 || !expected.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(tonic::Status::invalid_argument("invalid SHA-256 checksum"));
        }

        let original_destination = first.destination.clone();
        let total_size = first.total_size;
        let parent = destination
            .parent()
            .ok_or_else(|| tonic::Status::invalid_argument("destination has no parent"))?;
        tokio::fs::create_dir_all(parent).await.map_err(internal)?;
        let temp = parent.join(format!(
            ".{}.upload-{}",
            destination
                .file_name()
                .and_then(|v| v.to_str())
                .unwrap_or("file"),
            uuid::Uuid::new_v4()
        ));
        let mut file = tokio::fs::File::create(&temp).await.map_err(internal)?;
        let mut hasher = Sha256::new();
        let mut written = 0u64;
        let result = async {
            let mut chunk = first;
            loop {
                if chunk.server_id != self.config.server_id
                    || chunk.destination != original_destination
                    || chunk.total_size != total_size
                    || chunk.checksum_sha256.to_ascii_lowercase() != expected
                {
                    return Err(tonic::Status::invalid_argument("upload metadata changed"));
                }
                let next = written
                    .checked_add(chunk.data.len() as u64)
                    .ok_or_else(|| tonic::Status::invalid_argument("upload is too large"))?;
                if next > total_size {
                    return Err(tonic::Status::invalid_argument(
                        "upload exceeds declared size",
                    ));
                }
                file.write_all(&chunk.data).await.map_err(internal)?;
                hasher.update(&chunk.data);
                written = next;
                if chunk.finish {
                    break;
                }
                chunk = tokio::time::timeout(
                    std::time::Duration::from_secs(self.config.upload_timeout_seconds),
                    stream.next(),
                )
                .await
                .map_err(|_| tonic::Status::deadline_exceeded("upload timed out"))?
                .ok_or_else(|| tonic::Status::invalid_argument("upload ended before finish"))?
                .map_err(|e| tonic::Status::invalid_argument(format!("read upload: {e}")))?;
            }
            if written != total_size || format!("{:x}", hasher.finalize()) != expected {
                return Err(tonic::Status::invalid_argument(
                    "upload checksum or size mismatch",
                ));
            }
            file.flush().await.map_err(internal)?;
            tokio::fs::rename(&temp, &destination)
                .await
                .map_err(internal)?;
            Ok(proto::FileUploadResult {
                bytes_written: written,
                destination: destination.to_string_lossy().into_owned(),
            })
        }
        .await;
        if result.is_err() {
            let _ = tokio::fs::remove_file(&temp).await;
        }
        result
    }
}

fn internal(error: std::io::Error) -> tonic::Status {
    tonic::Status::internal(error.to_string())
}

fn validate_destination(raw: &str, roots: &[String]) -> Result<PathBuf, tonic::Status> {
    let path = Path::new(raw);
    if !path.is_absolute() {
        return Err(tonic::Status::invalid_argument(
            "destination must be absolute",
        ));
    }
    if path
        .components()
        .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        return Err(tonic::Status::permission_denied(
            "destination traversal is not allowed",
        ));
    }
    if !roots
        .iter()
        .map(Path::new)
        .any(|root| path.starts_with(root))
    {
        return Err(tonic::Status::permission_denied(
            "destination is outside upload roots",
        ));
    }
    Ok(path.to_path_buf())
}
