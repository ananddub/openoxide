use std::sync::Arc;

use dashmap::DashMap;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::services::observability::monitoring::proto::{
    shell_message, ShellClose, ShellInput, ShellMessage, ShellResize,
};

use super::events::TerminalSize;

/// Input side of a live terminal session, kept in the registry so the unary
/// `ExecutionCommand` RPC can reach the PTY after the streaming RPC opened it.
/// The agent sender is behind a mutex because `mpsc::Sender` is `!Sync` and
/// `DashMap` values must be `Sync`.
#[derive(Clone)]
pub enum TerminalSessionHandle {
    AgentShell {
        sender: Arc<tokio::sync::Mutex<mpsc::Sender<ShellMessage>>>,
    },
    RusshShell {
        terminal: Arc<os::ssh::RusshTerminal>,
    },
}

impl TerminalSessionHandle {
    pub async fn send_input(&self, data: &[u8]) -> Result<(), String> {
        match self {
            Self::AgentShell { sender } => {
                let message = ShellMessage {
                    payload: Some(shell_message::Payload::Input(ShellInput {
                        data: data.to_vec(),
                    })),
                };
                sender
                    .lock()
                    .await
                    .send(message)
                    .await
                    .map_err(|error| format!("could not write input to Agent shell: {error}"))
            }
            Self::RusshShell { terminal } => terminal
                .write(data)
                .await
                .map_err(|error| format!("could not write SSH terminal input: {error}")),
        }
    }

    pub async fn resize(&self, size: TerminalSize) -> Result<(), String> {
        match self {
            Self::AgentShell { sender } => {
                let message = ShellMessage {
                    payload: Some(shell_message::Payload::Resize(ShellResize {
                        cols: u32::from(size.cols),
                        rows: u32::from(size.rows),
                    })),
                };
                sender
                    .lock()
                    .await
                    .send(message)
                    .await
                    .map_err(|error| format!("could not resize Agent shell: {error}"))
            }
            Self::RusshShell { terminal } => terminal
                .resize(size.cols, size.rows)
                .await
                .map_err(|error| format!("could not resize SSH terminal: {error}")),
        }
    }

    async fn close_agent_shell(&self) {
        if let Self::AgentShell { sender } = self {
            let _ = sender
                .lock()
                .await
                .send(ShellMessage {
                    payload: Some(shell_message::Payload::Close(ShellClose {})),
                })
                .await;
        }
    }
}

pub struct TerminalSessionEntry {
    pub owner_user_id: i64,
    pub cancel: CancellationToken,
    pub session: TerminalSessionHandle,
}

impl TerminalSessionEntry {
    /// Cancel the pump, then ask the Agent shell to close (Russh terminals
    /// close when dropped).
    pub async fn terminate(self) {
        self.cancel.cancel();
        self.session.close_agent_shell().await;
    }
}

/// Live terminal sessions keyed by session id. All registry access (open,
/// command lookup, teardown) funnels through here.
pub struct TerminalSessionRegistry {
    entries: DashMap<String, TerminalSessionEntry>,
}

impl TerminalSessionRegistry {
    pub fn new() -> Self {
        Self {
            entries: DashMap::new(),
        }
    }

    pub fn insert(&self, session_id: String, entry: TerminalSessionEntry) {
        self.entries.insert(session_id, entry);
    }

    /// Owner id and PTY handle for the unary `ExecutionCommand` RPC, cloned
    /// out from under the map guard.
    pub fn command_target(&self, session_id: &str) -> Option<(i64, TerminalSessionHandle)> {
        let entry = self.entries.get(session_id)?;
        Some((entry.owner_user_id, entry.session.clone()))
    }

    /// Remove a session and tear down its PTY. A no-op when the session is
    /// already gone.
    pub async fn remove_and_terminate(&self, session_id: &str) {
        if let Some((_, entry)) = self.entries.remove(session_id) {
            entry.terminate().await;
        }
    }

    /// Remove a session without teardown. Used by the pumps once the PTY has
    /// naturally exited, and as the sync fallback when no runtime is
    /// available (process shutdown) — dropping the entry still closes the
    /// Agent request channel and the SSH terminal.
    pub fn remove(&self, session_id: &str) {
        self.entries.remove(session_id);
    }
}
