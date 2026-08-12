use axum::extract::Multipart;

use super::{ApplicationRecord, ApplicationService};
use crate::{
    core::cache::CacheKey,
    repository::ServerRepository,
    utils::{
        exec::{CommandExecutor, LocalExecutor},
        paths::openoxide_paths,
        upload::{
            MAX_UPLOAD_BYTES, extract_zip, sanitize_zip, stream_multipart_file, upload_via_rclone,
        },
    },
};

impl ApplicationService {
    pub async fn upload_drop_source(
        &self,
        id: i64,
        multipart: Multipart,
    ) -> Result<ApplicationRecord, String> {
        let application = self
            .repo_app
            .get_by_id(id)
            .await
            .map_err(|error| format!("database error: {error}"))?
            .ok_or_else(|| "application not found".to_owned())?;

        let upload = tempfile::NamedTempFile::new()
            .map_err(|error| format!("failed to create upload file: {error}"))?;
        let clean = tempfile::NamedTempFile::new()
            .map_err(|error| format!("failed to create sanitized archive: {error}"))?;
        let uploaded =
            stream_multipart_file(multipart, "file", upload.path(), MAX_UPLOAD_BYTES).await?;
        if !uploaded.filename.to_ascii_lowercase().ends_with(".zip") {
            return Err("application source upload must be a .zip archive".into());
        }
        sanitize_zip(upload.path(), clean.path()).await?;
        let build_path = uploaded
            .fields
            .get("drop_build_path")
            .map(String::as_str)
            .unwrap_or("/")
            .trim();
        if build_path.len() > 4096 || build_path.contains(['\0', '\r', '\n']) {
            return Err("invalid drop build path".into());
        }
        let build_path = if build_path.is_empty() {
            "/"
        } else {
            build_path
        };

        let destination = openoxide_paths().application_code(&application.app_name);
        match application.server_id {
            None => {
                extract_zip(
                    &CommandExecutor::Local(LocalExecutor::new()),
                    clean.path(),
                    &destination,
                )
                .await?;
            }
            Some(server_id) => {
                let servers = auto_di::resolve::<ServerRepository>()
                    .await
                    .map_err(|error| format!("failed to resolve server repository: {error}"))?;
                let remote_archive = format!(
                    "/tmp/openoxide-application-upload-{}.zip",
                    uuid::Uuid::new_v4()
                );
                let remote =
                    upload_via_rclone(servers.as_ref(), server_id, clean.path(), &remote_archive)
                        .await?;
                let executor = CommandExecutor::Remote(remote);
                let result = extract_zip(&executor, &remote_archive, &destination).await;
                let _ = crate::utils::os::OsCli::new(&executor)
                    .file(&remote_archive)
                    .delete()
                    .run()
                    .await;
                result?;
            }
        }

        self.repo_app
            .set_drop_source(id, build_path.to_owned())
            .await
            .map_err(|error| format!("failed to update application source: {error}"))?;
        self.cache.invalidate(&CacheKey::Application(id)).await;
        self.get_by_id(id)
            .await
            .map_err(|error| format!("failed to reload application: {error}"))
    }
}
