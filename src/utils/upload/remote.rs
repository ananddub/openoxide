use std::{io::Write, path::Path};

use crate::{
    repository::ServerRepository,
    utils::{
        exec::{CommandExecutor, LocalExecutor, RemoteExecutor, SshAuth, SshHostKey},
        rclone::{builder::RcloneBuilder, command::RcloneCommand, target::RcloneTarget},
    },
};

pub async fn upload_via_rclone(
    servers: &ServerRepository,
    server_id: i64,
    local_path: &Path,
    remote_path: &str,
) -> Result<RemoteExecutor, String> {
    let (host, port, username, private_key, public_key) = servers
        .get_direct_ssh_credentials(server_id)
        .await
        .map_err(|error| format!("failed to load remote credentials: {error}"))?
        .ok_or_else(|| "server or SSH key not found".to_owned())?;
    let port = u16::try_from(port).map_err(|_| "invalid SSH port".to_owned())?;

    let mut key = tempfile::NamedTempFile::new()
        .map_err(|error| format!("failed to create temporary SSH key: {error}"))?;
    key.write_all(private_key.as_bytes())
        .map_err(|error| format!("failed to write temporary SSH key: {error}"))?;
    key.flush()
        .map_err(|error| format!("failed to flush temporary SSH key: {error}"))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(key.path(), std::fs::Permissions::from_mode(0o600))
            .map_err(|error| format!("failed to protect temporary SSH key: {error}"))?;
    }

    RcloneBuilder::new(RcloneCommand::Copyto)
        .source(RcloneTarget::Local {
            path: local_path.to_string_lossy().into_owned(),
        })
        .destination(RcloneTarget::Sftp {
            host: host.clone(),
            port: Some(port),
            user: username.clone(),
            pass: None,
            key_file: Some(key.path().to_string_lossy().into_owned()),
            key_use_agent: false,
            path: remote_path.to_owned(),
        })
        .execute(&CommandExecutor::Local(LocalExecutor::new()))
        .await
        .map_err(|error| format!("rclone SFTP upload failed: {error}"))?;

    Ok(RemoteExecutor::new(
        host,
        port,
        username,
        SshAuth::key_pair(private_key, public_key),
        SshHostKey::InsecureAcceptAny,
    )
    .with_sudo())
}
