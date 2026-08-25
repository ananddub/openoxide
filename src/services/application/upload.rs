use axum::extract::Multipart;

use super::{ApplicationRecord, ApplicationService};
use crate::{
    core::cache::CacheKey,
    services::monitoring::monitoring_service::MonitoringService,
    utils::{
        exec::CommandExecutor,
        paths::openoxide_paths,
        upload::{MAX_UPLOAD_BYTES, extract_zip, stream_multipart_file},
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
        let uploaded =
            stream_multipart_file(multipart, "file", upload.path(), MAX_UPLOAD_BYTES).await?;
        if !uploaded.filename.to_ascii_lowercase().ends_with(".zip") {
            return Err("application source upload must be a .zip archive".into());
        }
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
        let server_id =
            crate::services::compose::remote::agent_server_id(application.server_id).await?;
        let monitoring = auto_di::resolve::<MonitoringService>()
            .await
            .map_err(|error| format!("failed to resolve agent service: {error}"))?;
        let upload_id = uuid::Uuid::new_v4();
        let remote_archive = format!("/tmp/openoxide-application-upload-{upload_id}.zip");
        let clean_archive = format!("/tmp/openoxide-application-clean-{upload_id}.zip");
        monitoring
            .upload_file(server_id, upload.path(), &remote_archive)
            .await?;
        let executor = CommandExecutor::Agent(monitoring.agent_executor(server_id));
        let os = crate::utils::os::OsCli::new(&executor);
        let result = async {
            os.archive(&remote_archive)
                .zip()
                .sanitize()
                .to(&clean_archive)
                .run()
                .await
                .map_err(|error| format!("ZIP validation failed: {error}"))?;
            extract_zip(&executor, &clean_archive, &destination).await
        }
        .await;
        let _ = os.file(&remote_archive).delete().run().await;
        let _ = os.file(&clean_archive).delete().run().await;
        result?;

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
