use crate::exec::{ExecError, ExecResult, SshAuth, SshHostKey};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct InMemorySshTerminal {
    session: ssh2::Session,
    channel: Arc<Mutex<ssh2::Channel>>,
    _tcp: TcpStream,
}

impl std::fmt::Debug for InMemorySshTerminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InMemorySshTerminal").finish()
    }
}

impl InMemorySshTerminal {
    pub fn connect(
        host: &str,
        port: u16,
        username: &str,
        auth: &SshAuth,
        _host_key: &SshHostKey,
        cols: u16,
        rows: u16,
        timeout: Duration,
    ) -> ExecResult<Self> {
        let addr = format!("{host}:{port}");
        let tcp = TcpStream::connect_timeout(
            &addr
                .parse()
                .map_err(|e| ExecError::Ssh(format!("Invalid SSH address {addr}: {e}")))?,
            timeout,
        )
        .map_err(|e| ExecError::Ssh(format!("TCP connect to {addr} failed: {e}")))?;

        tcp.set_read_timeout(Some(Duration::from_secs(300))).ok();
        tcp.set_write_timeout(Some(Duration::from_secs(30))).ok();

        let mut session = ssh2::Session::new()
            .map_err(|e| ExecError::Ssh(format!("Failed to create SSH2 session: {e}")))?;

        session.set_tcp_stream(tcp.try_clone().map_err(|e| ExecError::Ssh(e.to_string()))?);
        session
            .handshake()
            .map_err(|e| ExecError::Ssh(format!("SSH handshake with {addr} failed: {e}")))?;

        match auth {
            SshAuth::Password(password) => {
                session
                    .userauth_password(username, password)
                    .map_err(|e| ExecError::Ssh(format!("SSH password auth failed for {username}: {e}")))?;
            }
            SshAuth::KeyFile(path) => {
                session
                    .userauth_pubkey_file(username, None, path, None)
                    .map_err(|e| ExecError::Ssh(format!("SSH keyfile auth failed: {e}")))?;
            }
            SshAuth::KeyPair { private_key, passphrase, .. } => {
                let mut temp_file = tempfile::NamedTempFile::new()
                    .map_err(|e| ExecError::Ssh(format!("Failed to create temp key file: {e}")))?;
                temp_file
                    .write_all(private_key.as_bytes())
                    .map_err(|e| ExecError::Ssh(format!("Failed to write temp key file: {e}")))?;

                session
                    .userauth_pubkey_file(
                        username,
                        None,
                        temp_file.path(),
                        passphrase.as_deref(),
                    )
                    .map_err(|e| ExecError::Ssh(format!("SSH keypair auth failed: {e}")))?;
            }
            SshAuth::Agent | SshAuth::AgentWithSocket(_) => {
                session
                    .userauth_agent(username)
                    .map_err(|e| ExecError::Ssh(format!("SSH agent auth failed for {username}: {e}")))?;
            }
        }

        let mut channel = session
            .channel_session()
            .map_err(|e| ExecError::Ssh(format!("Failed to open SSH channel: {e}")))?;

        channel.handle_extended_data(ssh2::ExtendedData::Merge).ok();
        channel
            .request_pty(
                "xterm-256color",
                None,
                Some((cols as u32, rows as u32, 0, 0)),
            )
            .map_err(|e| ExecError::Ssh(format!("Failed to request PTY: {e}")))?;

        channel
            .shell()
            .map_err(|e| ExecError::Ssh(format!("Failed to request shell: {e}")))?;

        Ok(Self {
            session,
            channel: Arc::new(Mutex::new(channel)),
            _tcp: tcp,
        })
    }

    pub fn read(&self, buf: &mut [u8]) -> ExecResult<usize> {
        let mut channel = self
            .channel
            .lock()
            .map_err(|_| ExecError::Ssh("Channel lock poisoned".into()))?;
        channel
            .read(buf)
            .map_err(|e| ExecError::Ssh(format!("SSH terminal read error: {e}")))
    }

    pub fn write(&self, data: &[u8]) -> ExecResult<usize> {
        let mut channel = self
            .channel
            .lock()
            .map_err(|_| ExecError::Ssh("Channel lock poisoned".into()))?;
        channel
            .write(data)
            .map_err(|e| ExecError::Ssh(format!("SSH terminal write error: {e}")))
    }

    pub fn resize(&self, cols: u16, rows: u16) -> ExecResult<()> {
        let mut channel = self
            .channel
            .lock()
            .map_err(|_| ExecError::Ssh("Channel lock poisoned".into()))?;
        channel
            .request_pty_size(cols as u32, rows as u32, None, None)
            .map_err(|e| ExecError::Ssh(format!("SSH PTY resize failed: {e}")))
    }

    pub fn close(&self) {
        if let Ok(mut channel) = self.channel.lock() {
            let _ = channel.close();
            let _ = channel.wait_close();
        }
    }
}
