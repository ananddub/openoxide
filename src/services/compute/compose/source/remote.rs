use crate::services::observability::monitoring::monitoring_service::MonitoringService;
use crate::utils::exec::CommandExecutor;
use crate::utils::os::OsCli;
use std::path::Path;

/// Stream a ZIP to the server agent, then extract it through the same agent.
pub async fn deploy_zip_to_remote(
    zip_path: &Path,
    dest_path: &str,
    server_id: i64,
    uuid: &str,
) -> Result<(), String> {
    let monitoring = auto_di::resolve::<MonitoringService>()
        .await
        .map_err(|e| format!("DI error: {e}"))?;
    let remote_zip = format!("/tmp/upload-{uuid}.zip");
    let clean_zip = format!("/tmp/clean-{uuid}.zip");
    monitoring
        .upload_file(server_id, zip_path, &remote_zip)
        .await
        .map_err(|e| format!("Agent transfer failed: {e}"))?;
    let cmd = CommandExecutor::Agent(monitoring.agent_executor(server_id));

    let os = OsCli::new(&cmd);
    os.archive(&remote_zip)
        .zip()
        .sanitize()
        .to(&clean_zip)
        .run()
        .await
        .map_err(|e| format!("ZIP validation failed: {e}"))?;
    os.dir(dest_path)
        .create()
        .run()
        .await
        .map_err(|e| format!("Failed to create remote directory: {e}"))?;

    os.archive(&clean_zip)
        .zip()
        .extract_to(dest_path)
        .overwrite()
        .run()
        .await
        .map_err(|e| format!("Failed to unzip on remote: {e}"))?;

    let _ = os.file(&remote_zip).delete().run().await;
    let _ = os.file(&clean_zip).delete().run().await;

    Ok(())
}
