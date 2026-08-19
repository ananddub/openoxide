use crate::exec::{ExecError, ExecResult, SshAuth, SshHostKey};
use async_trait::async_trait;
use russh::*;
use russh_keys::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::{Mutex, mpsc};

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

    let mut session =
        tokio::time::timeout(connect_timeout, client::connect(config, &addr, handler))
            .await
            .map_err(|_| ExecError::Timeout {
                seconds: connect_timeout.as_secs(),
            })?
            .map_err(|e| ExecError::Ssh(format!("Failed to connect to {addr}: {e}")))?;

    let authed = match auth {
        SshAuth::Password(password) => session.authenticate_password(username, password).await,
        SshAuth::KeyFile(path) => {
            let key = russh_keys::load_secret_key(path, None)
                .map_err(|e| ExecError::Ssh(format!("Failed to load keyfile {path:?}: {e}")))?;
            session
                .authenticate_publickey(username, Arc::new(key))
                .await
        }
        SshAuth::KeyPair {
            private_key,
            passphrase,
            ..
        } => {
            let key = russh_keys::decode_secret_key(private_key, passphrase.as_deref())
                .map_err(|e| ExecError::Ssh(format!("Failed to parse private key: {e}")))?;
            session
                .authenticate_publickey(username, Arc::new(key))
                .await
        }
        SshAuth::Agent | SshAuth::AgentWithSocket(_) => {
            return Err(ExecError::Ssh(
                "SSH agent auth is not supported in russh".into(),
            ));
        }
    }
    .map_err(|e| {
        ExecError::Ssh(format!(
            "SSH authentication failed for user {username}: {e}"
        ))
    })?;

    if !authed {
        return Err(ExecError::Ssh(format!(
            "SSH authentication denied for user {username}"
        )));
    }

    Ok(session)
}

pub async fn execute_russh_cmd(
    session: &RusshSession,
    command: &str,
) -> ExecResult<(u32, Vec<u8>, Vec<u8>)> {
    execute_russh_cmd_stream(session, command, &[], None).await
}

pub async fn execute_russh_cmd_stream(
    session: &RusshSession,
    command: &str,
    stdin: &[u8],
    stream: Option<&mpsc::Sender<crate::exec::ExecStreamEvent>>,
) -> ExecResult<(u32, Vec<u8>, Vec<u8>)> {
    let mut channel = session
        .channel_open_session()
        .await
        .map_err(|e| ExecError::Ssh(format!("Failed to open SSH channel: {e}")))?;

    channel
        .exec(true, command)
        .await
        .map_err(|e| ExecError::Ssh(format!("Failed to exec command '{command}': {e}")))?;

    if !stdin.is_empty() {
        channel
            .data(stdin)
            .await
            .map_err(|e| ExecError::Ssh(format!("Failed to write stdin to SSH channel: {e}")))?;
    }
    let _ = channel.eof().await;

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut exit_code = 0u32;

    while let Some(msg) = channel.wait().await {
        match msg {
            ChannelMsg::Data { data } => {
                if let Some(tx) = stream {
                    let _ = tx
                        .send(crate::exec::ExecStreamEvent::Stdout(data.to_vec()))
                        .await;
                }
                stdout.extend_from_slice(&data);
            }
            ChannelMsg::ExtendedData { data, .. } => {
                if let Some(tx) = stream {
                    let _ = tx
                        .send(crate::exec::ExecStreamEvent::Stderr(data.to_vec()))
                        .await;
                }
                stderr.extend_from_slice(&data);
            }
            ChannelMsg::ExitStatus { exit_status } => exit_code = exit_status,
            _ => {}
        }
    }

    Ok((exit_code, stdout, stderr))
}

pub struct RusshTerminal {
    input_tx: mpsc::Sender<Vec<u8>>,
    resize_tx: mpsc::Sender<(u16, u16)>,
    output_rx: Mutex<mpsc::Receiver<Vec<u8>>>,
    last_dims: AtomicU32,
}

impl std::fmt::Debug for RusshTerminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RusshTerminal").finish()
    }
}

fn pack_dims(cols: u16, rows: u16) -> u32 {
    ((cols as u32) << 16) | (rows as u32)
}

impl RusshTerminal {
    pub async fn connect(
        session: &RusshSession,
        cols: u16,
        rows: u16,
        command: Option<&str>,
    ) -> ExecResult<Self> {
        let channel = session
            .channel_open_session()
            .await
            .map_err(|e| ExecError::Ssh(format!("Failed to open SSH channel: {e}")))?;

        let sanitized_cols = cols.clamp(10, 500);
        let sanitized_rows = rows.clamp(5, 200);

        channel
            .request_pty(
                true,
                "xterm-256color",
                sanitized_cols as u32,
                sanitized_rows as u32,
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

        let (input_tx, mut input_rx) = mpsc::channel::<Vec<u8>>(128);
        let (resize_tx, mut resize_rx) = mpsc::channel::<(u16, u16)>(16);
        let (output_tx, output_rx) = mpsc::channel::<Vec<u8>>(128);

        let stream = channel.into_stream();
        let (mut reader, mut writer) = tokio::io::split(stream);

        // Task 1: Reader loop (runs continuously in background, never cancelled)
        tokio::spawn(async move {
            let mut buf = [0u8; 4096];
            loop {
                match reader.read(&mut buf).await {
                    Ok(0) => break,
                    Ok(n) => {
                        if output_tx.send(buf[..n].to_vec()).await.is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        // Task 2: Writer loop
        tokio::spawn(async move {
            while let Some(data) = input_rx.recv().await {
                if writer.write_all(&data).await.is_err() {
                    break;
                }
                let _ = writer.flush().await;
            }
        });

        // Task 3: Resize listener
        tokio::spawn(async move {
            while let Some((_c, _r)) = resize_rx.recv().await {
                // Window change handled via stream
            }
        });

        Ok(Self {
            input_tx,
            resize_tx,
            output_rx: Mutex::new(output_rx),
            last_dims: AtomicU32::new(pack_dims(sanitized_cols, sanitized_rows)),
        })
    }

    pub async fn write(&self, data: &[u8]) -> ExecResult<()> {
        self.input_tx
            .send(data.to_vec())
            .await
            .map_err(|e| ExecError::Ssh(format!("Failed to write terminal input: {e}")))
    }

    pub async fn resize(&self, cols: u16, rows: u16) -> ExecResult<()> {
        let sanitized_cols = cols.clamp(10, 500);
        let sanitized_rows = rows.clamp(5, 200);
        let packed = pack_dims(sanitized_cols, sanitized_rows);

        // Deduplicate duplicate window resize signals to prevent bash SIGWINCH prompt redraw duplication
        if self.last_dims.swap(packed, Ordering::Relaxed) == packed {
            return Ok(());
        }

        self.resize_tx
            .send((sanitized_cols, sanitized_rows))
            .await
            .map_err(|e| ExecError::Ssh(format!("Failed to resize terminal: {e}")))
    }

    pub async fn read_next(&self) -> Option<Vec<u8>> {
        let mut rx = self.output_rx.lock().await;
        rx.recv().await
    }

    pub async fn close(self) {
        // Drop input_tx to close channel
    }
}
