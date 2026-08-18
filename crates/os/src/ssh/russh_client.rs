use crate::exec::{ExecError, ExecResult, SshAuth, SshHostKey};
use async_trait::async_trait;
use russh::*;
use russh_keys::*;
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct RusshHandler;

#[async_trait]
impl client::Handler for RusshHandler {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        _server_public_key: &key::PublicKey,
    ) -> Result<bool, Self::Error> {
        Ok(true)
    }
}

pub type RusshSession = client::Handle<RusshHandler>;

pub async fn connect_russh(
    host: &str,
    port: u16,
    username: &str,
    auth: &SshAuth,
    _host_key: &SshHostKey,
    connect_timeout: Duration,
) -> ExecResult<RusshSession> {
    let config = Arc::new(client::Config::default());
    let handler = RusshHandler;
    let addr = format!("{host}:{port}");

    let mut session = tokio::time::timeout(
        connect_timeout,
        client::connect(config, &addr, handler),
    )
    .await
    .map_err(|_| ExecError::Timeout { seconds: connect_timeout.as_secs() })?
    .map_err(|e| ExecError::Ssh(format!("Failed to connect to {addr}: {e}")))?;

    let authed = match auth {
        SshAuth::Password(password) => {
            session.authenticate_password(username, password).await
        }
        SshAuth::KeyFile(path) => {
            let key = russh_keys::load_secret_key(path, None)
                .map_err(|e| ExecError::Ssh(format!("Failed to load keyfile {path:?}: {e}")))?;
            session.authenticate_publickey(username, Arc::new(key)).await
        }
        SshAuth::KeyPair { private_key, passphrase, .. } => {
            let mut temp_file = tempfile::NamedTempFile::new()
                .map_err(|e| ExecError::Ssh(format!("Failed to create temp key file: {e}")))?;
            use std::io::Write;
            temp_file
                .write_all(private_key.as_bytes())
                .map_err(|e| ExecError::Ssh(format!("Failed to write temp key file: {e}")))?;

            let key = russh_keys::load_secret_key(temp_file.path(), passphrase.as_deref())
                .map_err(|e| ExecError::Ssh(format!("Failed to load keypair: {e}")))?;
            session.authenticate_publickey(username, Arc::new(key)).await
        }
        SshAuth::Agent | SshAuth::AgentWithSocket(_) => {
            return Err(ExecError::Ssh("SSH agent auth is not supported in russh".into()));
        }
    }
    .map_err(|e| ExecError::Ssh(format!("SSH authentication failed for user {username}: {e}")))?;

    if !authed {
        return Err(ExecError::Ssh(format!("SSH authentication denied for user {username}")));
    }

    Ok(session)
}

pub async fn execute_russh_cmd(
    session: &RusshSession,
    command: &str,
) -> ExecResult<(u32, Vec<u8>, Vec<u8>)> {
    let mut channel = session
        .channel_open_session()
        .await
        .map_err(|e| ExecError::Ssh(format!("Failed to open SSH channel: {e}")))?;

    channel
        .exec(true, command)
        .await
        .map_err(|e| ExecError::Ssh(format!("Failed to exec command '{command}': {e}")))?;

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut exit_code = 0u32;

    while let Some(msg) = channel.wait().await {
        match msg {
            ChannelMsg::Data { data } => stdout.extend_from_slice(&data),
            ChannelMsg::ExtendedData { data, .. } => stderr.extend_from_slice(&data),
            ChannelMsg::ExitStatus { exit_status } => exit_code = exit_status,
            _ => {}
        }
    }

    Ok((exit_code, stdout, stderr))
}

pub struct RusshTerminal {
    channel: Channel<client::Msg>,
}

impl std::fmt::Debug for RusshTerminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RusshTerminal").finish()
    }
}

impl RusshTerminal {
    pub async fn connect(
        session: &RusshSession,
        cols: u16,
        rows: u16,
        command: Option<&str>,
    ) -> ExecResult<Self> {
        let mut channel = session
            .channel_open_session()
            .await
            .map_err(|e| ExecError::Ssh(format!("Failed to open SSH channel: {e}")))?;

        channel
            .request_pty(
                true,
                "xterm-256color",
                cols as u32,
                rows as u32,
                0,
                0,
                &[],
            )
            .await
            .map_err(|e| ExecError::Ssh(format!("Failed to request PTY: {e}")))?;

        if let Some(cmd) = command {
            channel
                .exec(true, cmd)
                .await
                .map_err(|e| ExecError::Ssh(format!("Failed to exec command on PTY: {e}")))?;
        } else {
            channel
                .request_shell(true)
                .await
                .map_err(|e| ExecError::Ssh(format!("Failed to request shell: {e}")))?;
        }

        Ok(Self { channel })
    }

    pub async fn write(&mut self, data: &[u8]) -> ExecResult<()> {
        self.channel
            .data(data)
            .await
            .map_err(|e| ExecError::Ssh(format!("Failed to write to SSH channel: {e}")))
    }

    pub async fn resize(&mut self, cols: u16, rows: u16) -> ExecResult<()> {
        self.channel
            .window_change(cols as u32, rows as u32, 0, 0)
            .await
            .map_err(|e| ExecError::Ssh(format!("Failed to resize SSH window: {e}")))
    }

    pub async fn read_next(&mut self) -> Option<Vec<u8>> {
        while let Some(msg) = self.channel.wait().await {
            match msg {
                ChannelMsg::Data { data } => return Some(data.to_vec()),
                ChannelMsg::ExtendedData { data, .. } => return Some(data.to_vec()),
                _ => {}
            }
        }
        None
    }

    pub async fn close(mut self) {
        let _ = self.channel.close().await;
    }
}
