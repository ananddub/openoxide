use crate::exec::{ExecError, ExecResult, SshAuth, SshHostKey};
use async_trait::async_trait;
use russh::*;
use russh_keys::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::{Mutex, mpsc};

/// Handler that enforces the configured host-key policy during the SSH
/// handshake. russh invokes `check_server_key` exactly once per connection
/// (before `NewKeys`); returning `Ok(false)` aborts with `Error::UnknownKey`.
#[derive(Clone)]
pub struct RusshHandler {
    /// `Some(fingerprint)` pins the server key; `None` accepts any (explicit
    /// `InsecureAcceptAny` policy only).
    pinned: Option<String>,
    /// Optional shared slot that records the server's observed key during the
    /// handshake. Used by trust-on-first-capture: the setup/audit flow connects
    /// with `InsecureAcceptAny` and reads the key back to persist it as the pin
    /// (fingerprint, for russh) and as an OpenSSH `known_hosts` line (for the
    /// rsync subprocess, which shells out to system `ssh`).
    observed: Option<Arc<Mutex<Option<ServerHostKey>>>>,
}

/// A captured SSH host key. `fingerprint` is the raw base64 SHA-256 digest used
/// to pin russh connections; `openssh_key` is the host-agnostic
/// `"<algorithm> <base64-blob>"` form, prefixed with the connect host at use
/// time to build a `known_hosts` file for rsync.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ServerHostKey {
    pub fingerprint: String,
    pub openssh_key: String,
}

/// Render a public key in OpenSSH `known_hosts` key form (without the host
/// token): `"<algorithm> <base64(wire-blob)>"`.
fn openssh_key_form(key: &key::PublicKey) -> String {
    use base64::Engine as _;
    let blob = key.public_key_bytes();
    let encoded = base64::engine::general_purpose::STANDARD.encode(&blob);
    format!("{} {}", key.name(), encoded)
}

#[async_trait]
impl client::Handler for RusshHandler {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        server_public_key: &key::PublicKey,
    ) -> Result<bool, Self::Error> {
        let actual = server_public_key.fingerprint();
        if let Some(slot) = &self.observed {
            *slot.lock().await = Some(ServerHostKey {
                fingerprint: actual.clone(),
                openssh_key: openssh_key_form(server_public_key),
            });
        }
        match &self.pinned {
            None => Ok(true),
            Some(expected) => {
                let ok = fingerprint_matches(expected, &actual);
                if !ok {
                    tracing::error!(
                        expected = %expected,
                        actual = %actual,
                        "russh: server host key fingerprint does not match the pinned value; rejecting connection"
                    );
                }
                Ok(ok)
            }
        }
    }
}

/// Normalise a base64 SHA-256 host-key fingerprint for comparison: trim
/// whitespace, drop an optional `SHA256:` prefix, and drop base64 padding.
/// `PublicKey::fingerprint()` yields the raw `BASE64_NOPAD` digest, while the
/// stored pin may arrive with or without the prefix/padding depending on which
/// tool captured it (mirrors the normalisation in `ssh::builder`).
fn normalize_fingerprint(s: &str) -> String {
    let s = s.trim();
    let s = s.strip_prefix("SHA256:").unwrap_or(s).trim();
    s.trim_end_matches('=').to_string()
}

/// Compare a pinned fingerprint against a computed one. Fails closed: an empty
/// (or whitespace/prefix-only) expected value never matches.
fn fingerprint_matches(expected: &str, actual: &str) -> bool {
    let expected = normalize_fingerprint(expected);
    !expected.is_empty() && expected == normalize_fingerprint(actual)
}

pub type RusshSession = client::Handle<RusshHandler>;

pub async fn connect_russh(
    host: &str,
    port: u16,
    username: &str,
    auth: &SshAuth,
    host_key: &SshHostKey,
    connect_timeout: Duration,
) -> ExecResult<RusshSession> {
    // `connect_timeout` only bounded the TCP+KEX handshake; authentication
    // (a stalled sshd, PAM, a slow key check) could still block forever.
    // Bound the whole connect+auth sequence.
    let budget = connect_timeout.saturating_mul(2);
    tokio::time::timeout(
        budget,
        connect_russh_inner(host, port, username, auth, host_key, connect_timeout),
    )
    .await
    .map_err(|_| ExecError::Timeout {
        seconds: budget.as_secs(),
    })?
}

async fn connect_russh_inner(
    host: &str,
    port: u16,
    username: &str,
    auth: &SshAuth,
    host_key: &SshHostKey,
    connect_timeout: Duration,
) -> ExecResult<RusshSession> {
    let config = Arc::new(client::Config {
        // Keep long-running sessions (log follows, deploys) alive through
        // idle NAT/firewall timeouts and never drop them for inactivity.
        keepalive_interval: Some(Duration::from_secs(15)),
        keepalive_max: 3,
        inactivity_timeout: None,
        ..Default::default()
    });
    let pinned = match host_key {
        SshHostKey::InsecureAcceptAny => None,
        SshHostKey::PinnedSha256(fingerprint) => Some(fingerprint.clone()),
    };
    let handler = RusshHandler {
        pinned,
        observed: None,
    };
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

/// Open a throwaway handshake (no authentication) purely to observe the
/// server's host key. Used for trust-on-first-capture during initial server
/// setup/audit when no pin is stored yet. `check_server_key` fires before
/// authentication, so the key is available as soon as `client::connect`
/// returns; the session is then dropped.
pub async fn capture_host_key(
    host: &str,
    port: u16,
    connect_timeout: Duration,
) -> ExecResult<ServerHostKey> {
    let config = Arc::new(client::Config::default());
    let observed: Arc<Mutex<Option<ServerHostKey>>> = Arc::new(Mutex::new(None));
    let handler = RusshHandler {
        pinned: None,
        observed: Some(observed.clone()),
    };
    let addr = format!("{host}:{port}");

    let session = tokio::time::timeout(connect_timeout, client::connect(config, &addr, handler))
        .await
        .map_err(|_| ExecError::Timeout {
            seconds: connect_timeout.as_secs(),
        })?
        .map_err(|e| ExecError::Ssh(format!("Failed to connect to {addr}: {e}")))?;

    let captured = observed
        .lock()
        .await
        .take()
        .ok_or_else(|| ExecError::Ssh(format!("No host key observed from {addr}")))?;

    drop(session);
    Ok(captured)
}

/// Convenience wrapper over [`capture_host_key`] returning only the SHA-256
/// fingerprint used to pin russh connections.
pub async fn capture_host_key_fingerprint(
    host: &str,
    port: u16,
    connect_timeout: Duration,
) -> ExecResult<String> {
    Ok(capture_host_key(host, port, connect_timeout)
        .await?
        .fingerprint)
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
        .map_err(|e| ExecError::ChannelOpen(format!("Failed to open SSH channel: {e}")))?;

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
    // A channel that closes without an exit status means the connection died
    // (keepalive failure, sshd killed, network drop) — not that the command
    // succeeded. Defaulting to 0 turned a truncated command into a success.
    let mut exit_seen = false;

    // In stream mode the consumer already receives every chunk, so the
    // mirrored Vec is only error-path context.  Without a cap a long-lived
    // stream (`docker logs -f` over SSH) accumulates the whole output and
    // grows until OOM.  Non-streaming callers get the Vec as the only result
    // and must keep it untruncated.
    const MIRROR_CAP: usize = 256 * 1024;

    while let Some(msg) = channel.wait().await {
        match msg {
            ChannelMsg::Data { data } => {
                if let Some(tx) = stream {
                    let _ = tx
                        .send(crate::exec::ExecStreamEvent::Stdout(data.to_vec()))
                        .await;
                    if stdout.len() < MIRROR_CAP {
                        let take = (MIRROR_CAP - stdout.len()).min(data.len());
                        stdout.extend_from_slice(&data[..take]);
                    }
                } else {
                    stdout.extend_from_slice(&data);
                }
            }
            ChannelMsg::ExtendedData { data, .. } => {
                if let Some(tx) = stream {
                    let _ = tx
                        .send(crate::exec::ExecStreamEvent::Stderr(data.to_vec()))
                        .await;
                    if stderr.len() < MIRROR_CAP {
                        let take = (MIRROR_CAP - stderr.len()).min(data.len());
                        stderr.extend_from_slice(&data[..take]);
                    }
                } else {
                    stderr.extend_from_slice(&data);
                }
            }
            ChannelMsg::ExitStatus { exit_status } => {
                exit_code = exit_status;
                exit_seen = true;
            }
            ChannelMsg::ExitSignal { .. } => {
                // Killed by a signal: a definite (non-zero) outcome.
                exit_code = 255;
                exit_seen = true;
            }
            _ => {}
        }
    }

    if !exit_seen {
        return Err(ExecError::Ssh(format!(
            "SSH connection closed before '{command}' reported an exit status"
        )));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pinned_fingerprint_matches_regardless_of_prefix_and_padding() {
        // PublicKey::fingerprint() yields raw BASE64_NOPAD, e.g. "abc...XY".
        let actual = "n9Sl8Zj5FR7A0PBdBrJkoEsD5Kd2kPi07b0Eeu9wN5s";
        assert!(fingerprint_matches(actual, actual));
        assert!(fingerprint_matches(&format!("SHA256:{actual}"), actual));
        assert!(fingerprint_matches(&format!("  {actual}  "), actual));
        assert!(fingerprint_matches(actual, &format!("{actual}==")));
    }

    #[test]
    fn mismatched_fingerprint_is_rejected() {
        assert!(!fingerprint_matches(
            "n9Sl8Zj5FR7A0PBdBrJkoEsD5Kd2kPi07b0Eeu9wN5s",
            "differentdifferentdifferentdifferentdiffere"
        ));
    }

    #[test]
    fn empty_pin_fails_closed() {
        // An empty / prefix-only stored pin must never accept an arbitrary key.
        assert!(!fingerprint_matches(
            "",
            "n9Sl8Zj5FR7A0PBdBrJkoEsD5Kd2kPi07b0E"
        ));
        assert!(!fingerprint_matches(
            "SHA256:",
            "n9Sl8Zj5FR7A0PBdBrJkoEsD5Kd2kPi07b0E"
        ));
        assert!(!fingerprint_matches(
            "   ",
            "n9Sl8Zj5FR7A0PBdBrJkoEsD5Kd2kPi07b0E"
        ));
    }
}
