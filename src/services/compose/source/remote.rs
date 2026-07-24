use crate::repository::ServerRepository;
use crate::services::compose::remote::remote_executor;
use crate::utils::exec::{CommandExecutor, LocalExecutor};
use crate::utils::rclone::{builder::RcloneBuilder, command::RcloneCommand, target::RcloneTarget};
use crate::utils::zip::ZipBuilder;
use std::path::Path;
use std::sync::Arc;

/// Transfer a ZIP to a remote server via rclone SFTP, then extract it via SSH.
pub async fn deploy_zip_to_remote(
    db: &Arc<sqlx::SqlitePool>,
    zip_path: &Path,
    dest_path: &str,
    server_id: i64,
    key_path: &Path,
    uuid: &str,
) -> Result<(), String> {
    let repo = auto_di::resolve::<ServerRepository>()
        .await
        .map_err(|e| format!("DI error: {e}"))?;
    let (ip, port, username, private_key, _pub_key) = repo
        .get_ssh_credentials(server_id)
        .await
        .map_err(|e| format!("Failed to load server credentials: {e}"))?
        .ok_or_else(|| "Server not found".to_string())?;

    std::fs::write(key_path, &private_key)
        .map_err(|e| format!("Failed to write private key: {e}"))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(key_path, std::fs::Permissions::from_mode(0o600))
            .map_err(|e| format!("Failed to set key permissions: {e}"))?;
    }

    let local = CommandExecutor::Local(LocalExecutor::new());
    let remote_zip = format!("/tmp/upload-{uuid}.zip");

    RcloneBuilder::new(RcloneCommand::Copyto)
        .source(RcloneTarget::Local {
            path: zip_path.to_string_lossy().into_owned(),
        })
        .destination(RcloneTarget::Sftp {
            host: ip.clone(),
            port: Some(port as u16),
            user: username.clone(),
            pass: None,
            key_file: Some(key_path.to_string_lossy().into_owned()),
            key_use_agent: false,
            path: remote_zip.clone(),
        })
        .execute(&local)
        .await
        .map_err(|e| format!("Rclone transfer failed: {e}"))?;

    let ssh = remote_executor(db, server_id)
        .await
        .map_err(|e| format!("Failed to build SSH executor: {e}"))?;
    let cmd = CommandExecutor::Remote(ssh);

    cmd.run("mkdir", &["-p", dest_path])
        .await
        .map_err(|e| format!("Failed to create remote directory: {e}"))?;

    ZipBuilder::new(&cmd)
        .source(&remote_zip)
        .destination(dest_path)
        .overwrite()
        .unzip()
        .await
        .map_err(|e| format!("Failed to unzip on remote: {e}"))?;

    let _ = cmd.run("rm", &["-f", &remote_zip]).await;

    Ok(())
}
