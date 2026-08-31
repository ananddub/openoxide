use std::sync::Arc;

use auto_di::{resolve, singleton};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::api::connect::generated::openoxide::api::v1::{
    OpenTerminalRequest, TerminalEvent, TerminalKind, execution_command_request,
};
use crate::services::observability::monitoring::proto::{ShellAction, ShellMessage};
use crate::services::observability::monitoring::service::MonitoringService;

use super::events::{SessionEvents, TerminalSize};
use super::pump::TerminalPump;
use super::registry::{TerminalSessionEntry, TerminalSessionHandle, TerminalSessionRegistry};
use super::types::{TerminalCommandError, TerminalSession};

pub struct TerminalService {
    sessions: Arc<TerminalSessionRegistry>,
}

#[singleton]
impl TerminalService {
    fn new() -> Self {
        Self {
            sessions: Arc::new(TerminalSessionRegistry::new()),
        }
    }

    /// Open a terminal session and start streaming its events. Late startup
    /// failures surface as an Error event inside the stream so the terminal
    /// UI sees the same shape as runtime errors. Dropping the returned
    /// session tears it down.
    pub async fn open(
        &self,
        kind: TerminalKind,
        request: OpenTerminalRequest,
        owner_user_id: i64,
    ) -> TerminalSession {
        let size = TerminalSize::from_proto(request.cols, request.rows);
        let session_id = uuid::Uuid::new_v4().to_string();
        let (events_tx, events_rx) = mpsc::channel::<TerminalEvent>(64);
        let events = SessionEvents::new(session_id.clone(), events_tx);
        let cancel = CancellationToken::new();

        let setup = match kind {
            TerminalKind::Docker => Self::open_agent_session(&request, size).await,
            TerminalKind::RemoteServer => Self::open_russh_session(&request, size).await,
            TerminalKind::Unspecified => Err("terminal kind is required".to_owned()),
        };

        match setup {
            Ok((handle, pump, host)) => {
                self.sessions.insert(
                    session_id.clone(),
                    TerminalSessionEntry {
                        owner_user_id,
                        cancel: cancel.clone(),
                        session: handle,
                    },
                );
                let _ = events.emit_started(kind, host).await;
                pump.spawn(events, cancel, Arc::clone(&self.sessions));
            }
            Err(message) => {
                tracing::warn!("terminal session failed to open: {message}");
                let _ = events.emit_error(message).await;
            }
        }

        TerminalSession::new(session_id, events_rx, Arc::clone(&self.sessions))
    }

    /// Run one client command (input, resize, stop) against a live session.
    /// Commands for closed sessions succeed silently: the browser can flush
    /// queued input after the terminal already ended and streamed its error.
    pub async fn command(
        &self,
        session_id: &str,
        owner_user_id: i64,
        command: execution_command_request::Command,
    ) -> Result<(), TerminalCommandError> {
        let Some((owner, handle)) = self.sessions.command_target(session_id) else {
            tracing::debug!(session_id = %session_id, "terminal command for closed session");
            return Ok(());
        };

        if owner != owner_user_id {
            return Err(TerminalCommandError::PermissionDenied);
        }

        match command {
            execution_command_request::Command::Input(input) => handle
                .send_input(&input.data)
                .await
                .map_err(TerminalCommandError::Failed),
            execution_command_request::Command::Resize(resize) => {
                let size = TerminalSize::from_proto(Some(resize.cols), Some(resize.rows));
                handle
                    .resize(size)
                    .await
                    .map_err(TerminalCommandError::Failed)
            }
            execution_command_request::Command::Stop(_) => {
                self.close(session_id).await;
                Ok(())
            }
        }
    }

    /// Full teardown for an ended session: cancel the pump, close the PTY
    /// and deregister it.
    pub async fn close(&self, session_id: &str) {
        self.sessions.remove_and_terminate(session_id).await;
    }

    async fn open_agent_session(
        request: &OpenTerminalRequest,
        size: TerminalSize,
    ) -> Result<(TerminalSessionHandle, TerminalPump, Option<String>), String> {
        let server_id = crate::services::compute::compose::remote::agent_server_id(request.server_id)
            .await
            .map_err(|error| format!("could not resolve Agent server: {error}"))?;
        let monitoring = resolve::<MonitoringService>()
            .await
            .map_err(|error| format!("could not resolve Agent service: {error}"))?;
        let shell = SanitizedShell::parse(request.shell.as_deref(), "sh");
        let open = ShellMessage {
            server_id,
            action: ShellAction::Open as i32,
            container: request
                .container
                .clone()
                .unwrap_or_else(|| "app".to_owned()),
            shell: shell.as_str().to_owned(),
            cols: size.cols as u32,
            rows: size.rows as u32,
            ..Default::default()
        };
        let (sender, stream) = monitoring
            .open_agent_shell(server_id, open)
            .await
            .map_err(|error| format!("could not start Agent shell: {error}"))?;
        Ok((
            TerminalSessionHandle::AgentShell {
                sender: Arc::new(tokio::sync::Mutex::new(sender)),
            },
            TerminalPump::Agent { stream },
            Some("agent".to_owned()),
        ))
    }

    async fn open_russh_session(
        request: &OpenTerminalRequest,
        size: TerminalSize,
    ) -> Result<(TerminalSessionHandle, TerminalPump, Option<String>), String> {
        use os::exec::{SshAuth, SshHostKey};
        let server_id = request
            .server_id
            .ok_or("a remote server is required for the SSH terminal")?;
        let queries = resolve::<crate::db::queries::FlashQueries>()
            .await
            .map_err(|error| format!("could not resolve database queries: {error}"))?;
        let credentials =
            crate::services::infrastructure::server::ssh_credentials(&queries, server_id)
                .await
                .map_err(|error| format!("could not load SSH credentials: {error}"))?;
        let Some((host, port, username, private_key, public_key)) = credentials else {
            return Err("server not found or SSH credentials are missing".to_owned());
        };
        let port = u16::try_from(port).map_err(|_| "SSH port is invalid".to_owned())?;
        let session = os::ssh::connect_russh(
            &host,
            port,
            &username,
            &SshAuth::key_pair(private_key, public_key),
            &SshHostKey::InsecureAcceptAny,
            std::time::Duration::from_secs(15),
        )
        .await
        .map_err(|error| format!("SSH connection failed: {error}"))?;
        let shell_command = match request.shell.as_deref() {
            Some("sh") | Some("/bin/sh") | Some("bash") | Some("/bin/bash") | Some("") | None => {
                None
            }
            Some(command) => Some(command),
        };
        let terminal = os::ssh::RusshTerminal::connect(&session, size.cols, size.rows, shell_command)
            .await
            .map_err(|error| format!("SSH terminal failed to open: {error}"))?;
        let terminal = Arc::new(terminal);
        Ok((
            TerminalSessionHandle::RusshShell {
                terminal: Arc::clone(&terminal),
            },
            TerminalPump::Russh { terminal },
            Some(host),
        ))
    }
}

/// Shell program allowed into the Agent shell request, restricted to a
/// conservative character set.
enum SanitizedShell {
    Sh,
    Bash,
    Custom(String),
}

impl SanitizedShell {
    fn parse(value: Option<&str>, default_shell: &'static str) -> Self {
        let value = value.unwrap_or(default_shell).trim();
        match value {
            "sh" | "/bin/sh" => Self::Sh,
            "bash" | "/bin/bash" => Self::Bash,
            other => {
                let clean: String = other
                    .chars()
                    .filter(|c| c.is_alphanumeric() || *c == '/' || *c == '-' || *c == '_')
                    .collect();
                if clean.is_empty() {
                    if default_shell == "bash" {
                        Self::Bash
                    } else {
                        Self::Sh
                    }
                } else {
                    Self::Custom(clean)
                }
            }
        }
    }

    fn as_str(&self) -> &str {
        match self {
            Self::Sh => "sh",
            Self::Bash => "bash",
            Self::Custom(value) => value,
        }
    }
}
