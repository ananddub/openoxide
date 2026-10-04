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

/// Deletes the in-flight upload temp file when the guard drops — covering the
/// paths a plain `if result.is_err()` misses: a client disconnect mid-stream
/// (tonic drops this handler's future) or a panic. `mem::forget` on the
/// completed rename keeps the file.
struct TempUploadGuard {
    path: PathBuf,
}

impl TempUploadGuard {
    fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Drop for TempUploadGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

#[singleton]
impl FileTransferService {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn agent_info(&self) -> proto::AgentInfoResponse {
        proto::AgentInfoResponse {
            protocol_version: "3".into(),
            agent_version: env!("CARGO_PKG_VERSION").into(),
            capabilities: vec!["file_upload".into()],
            max_upload_bytes: self.config.max_upload_bytes,
            upload_roots: self.config.upload_roots.clone(),
            // Filled live by the update probe in the gRPC handler.
            self_image_id: String::new(),
            self_server_id: 0,
            self_running: false,
            latest_image_id: String::new(),
        }
    }

    pub async fn upload(
        &self,
        mut stream: tonic::Streaming<proto::FileUploadMessage>,
    ) -> Result<proto::FileUploadResult, tonic::Status> {
        let first = tokio::time::timeout(
            std::time::Duration::from_secs(self.config.upload_timeout_seconds),
            stream.next(),
        )
        .await
        .map_err(|_| tonic::Status::deadline_exceeded("upload timed out"))?
        .ok_or_else(|| tonic::Status::invalid_argument("upload stream is empty"))?
        .map_err(|e| tonic::Status::invalid_argument(format!("read upload: {e}")))?;

        // The first message carries all metadata; the rest are raw chunks.
        let header = match first.message {
            Some(proto::file_upload_message::Message::Header(header)) => header,
            _ => {
                return Err(tonic::Status::invalid_argument(
                    "upload must start with a header message",
                ));
            }
        };
        // Same rule as the gRPC query handlers: -1 marks the local Agent,
        // which serves whatever server row the panel maps it to.
        if header.server_id <= 0
            || (self.config.server_id != -1 && header.server_id != self.config.server_id)
        {
            return Err(tonic::Status::not_found(
                "upload server_id does not belong to this agent",
            ));
        }
        let destination = validate_destination(&header.destination, &self.config.upload_roots)?;
        if header.total_size == 0 || header.total_size > self.config.max_upload_bytes {
            return Err(tonic::Status::invalid_argument("invalid upload size"));
        }
        let expected = header.checksum_sha256.trim().to_ascii_lowercase();
        if expected.len() != 64 || !expected.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(tonic::Status::invalid_argument("invalid SHA-256 checksum"));
        }

        let total_size = header.total_size;
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
        // RAII cleanup: the plain error path below cannot run when tonic
        // drops this future on a mid-upload client disconnect, so the guard
        // deletes the temp file on ANY unwind; the completed rename forgets
        // the guard instead.
        let mut guard = Some(TempUploadGuard::new(temp.clone()));
        let mut hasher = Sha256::new();
        let mut written = 0u64;
        let result = async {
            // End-of-stream (rather than a flag) marks completion: the byte
            // count must match the header's declared total.
            while let Some(chunk) = tokio::time::timeout(
                std::time::Duration::from_secs(self.config.upload_timeout_seconds),
                stream.next(),
            )
            .await
            .map_err(|_| tonic::Status::deadline_exceeded("upload timed out"))?
            {
                let chunk = chunk
                    .map_err(|e| tonic::Status::invalid_argument(format!("read upload: {e}")))?;
                let data = match chunk.message {
                    Some(proto::file_upload_message::Message::Data(data)) => data,
                    _ => {
                        return Err(tonic::Status::invalid_argument(
                            "upload metadata must not change mid-stream",
                        ));
                    }
                };
                let next = written
                    .checked_add(data.len() as u64)
                    .ok_or_else(|| tonic::Status::invalid_argument("upload is too large"))?;
                if next > total_size {
                    return Err(tonic::Status::invalid_argument(
                        "upload exceeds declared size",
                    ));
                }
                file.write_all(&data).await.map_err(internal)?;
                hasher.update(&data);
                written = next;
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
            // Only a completed rename means "keep the file"; until this point
            // the guard's Drop removes the temp file on error or cancellation.
            if let Some(guard) = guard.take() {
                std::mem::forget(guard);
            }
            Ok(proto::FileUploadResult {
                bytes_written: written,
                destination: destination.to_string_lossy().into_owned(),
            })
        }
        .await;
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
