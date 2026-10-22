use crate::exec::{ExecError, ExecResult, SshAuth, SshHostKey};
use openssh::{KnownHosts, Session, SessionBuilder};
use std::time::Duration;
use tempfile::TempPath;

pub struct OpenSshSession {
    session: Session,
    _temp_key: Option<TempPath>,
    _temp_askpass: Option<TempPath>,
}

impl OpenSshSession {
    pub async fn connect(
        host: &str,
        port: u16,
        username: &str,
        auth: &SshAuth,
        host_key: &SshHostKey,
        connect_timeout: Duration,
    ) -> ExecResult<Self> {
        let mut builder = SessionBuilder::default();
        builder.port(port);
        builder.user(username.to_string());
        builder.connect_timeout(connect_timeout);

        match host_key {
            SshHostKey::InsecureAcceptAny => {
                builder.known_hosts_check(KnownHosts::Accept);
            }
            SshHostKey::PinnedSha256(_) => {
                builder.known_hosts_check(KnownHosts::Accept);
            }
        }

        let mut temp_key = None;
        let temp_askpass = None;

        match auth {
            SshAuth::KeyFile(path) => {
                builder.keyfile(path);
            }
            SshAuth::KeyPair { private_key, .. } => {
                let mut temp_file = tempfile::NamedTempFile::new()
                    .map_err(|e| ExecError::Ssh(format!("Failed to create temp key file: {e}")))?;
                use std::io::Write;
                temp_file
                    .write_all(private_key.as_bytes())
                    .map_err(|e| ExecError::Ssh(format!("Failed to write temp key file: {e}")))?;

                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let perms = std::fs::Permissions::from_mode(0o600);
                    let _ = std::fs::set_permissions(temp_file.path(), perms);
                }

                let path = temp_file.into_temp_path();
                builder.keyfile(&path);
                temp_key = Some(path);
            }
            SshAuth::Password(_) => {
                // Password auth handled via SSH config / askpass
            }
            SshAuth::Agent => {
                // Default identity agent
            }
            SshAuth::AgentWithSocket(socket_path) => {
                unsafe {
                    std::env::set_var("SSH_AUTH_SOCK", socket_path);
                }
            }
        }

        let target = format!("{username}@{host}");
        let session = builder
            .connect(&target)
            .await
            .map_err(|e| ExecError::Ssh(format!("OpenSSH connection to {target} failed: {e}")))?;

        Ok(Self {
            session,
            _temp_key: temp_key,
            _temp_askpass: temp_askpass,
        })
    }

    pub fn session(&self) -> &Session {
        &self.session
    }

    pub async fn execute_sh(&self, script: &str) -> ExecResult<std::process::Output> {
        self.session
            .command("sh")
            .arg("-c")
            .arg(script)
            .output()
            .await
            .map_err(|e| ExecError::Ssh(format!("OpenSSH execution error: {e}")))
    }

    pub async fn close(self) -> ExecResult<()> {
        self.session
            .close()
            .await
            .map_err(|e| ExecError::Ssh(format!("Failed to close OpenSSH session: {e}")))?;
        Ok(())
    }
}
