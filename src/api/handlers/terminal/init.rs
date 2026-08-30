use std::sync::Arc;

use auto_di::{resolve, singleton};
use dashmap::DashMap;

use crate::api::connect::generated::openoxide::api::v1::OpenTerminalRequest;
use crate::services::observability::monitoring::monitoring_service::MonitoringService;
use crate::services::observability::monitoring::proto::{ShellAction, ShellMessage};

use super::helper::{SanitizedShell, TerminalPump, TerminalSessionEntry, TerminalSessionHandle};

pub struct TerminalController {
    pub(super) sessions: Arc<DashMap<String, TerminalSessionEntry>>,
}

#[singleton]
impl TerminalController {
    fn new() -> Self {
        Self {
            sessions: Arc::new(DashMap::new()),
        }
    }
}

// Controller extensions — non-gRPC helper methods live here, not in controller.rs.
impl TerminalController {
    pub(super) async fn terminate(entry: TerminalSessionEntry) {
        entry.cancel.cancel();
        if let TerminalSessionHandle::AgentShell { sender } = entry.session {
            let sender = sender.lock().await;
            let _ = sender
                .send(ShellMessage {
                    action: ShellAction::Close as i32,
                    ..Default::default()
                })
                .await;
        }
    }

    pub(super) async fn open_agent_session(
        request: &OpenTerminalRequest,
        cols: u16,
        rows: u16,
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
            cols: cols as u32,
            rows: rows as u32,
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

    pub(super) async fn open_russh_session(
        request: &OpenTerminalRequest,
        cols: u16,
        rows: u16,
    ) -> Result<(TerminalSessionHandle, TerminalPump, Option<String>), String> {
        use os::exec::{SshAuth, SshHostKey};
        let server_id = request
            .server_id
            .ok_or("a remote server is required for the SSH terminal")?;
        let queries = resolve::<crate::db::queries::FlashQueries>()
            .await
            .map_err(|error| format!("could not resolve database queries: {error}"))?;
        let credentials = crate::services::infrastructure::server::ssh_credentials(&queries, server_id)
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
        let terminal = os::ssh::RusshTerminal::connect(&session, cols, rows, shell_command)
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
