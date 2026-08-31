use std::sync::Arc;

use dashmap::DashMap;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::services::observability::monitoring::proto::{ShellAction, ShellMessage};

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

async fn agent_message(
    sender: &tokio::sync::Mutex<mpsc::Sender<ShellMessage>>,
    action: ShellAction,
    data: Vec<u8>,
    size: TerminalSize,
    what: &str,
) -> Result<(), String> {
    let sender = sender.lock().await;
    sender
        .send(ShellMessage {
            action: action as i32,
            data,
            cols: size.cols as u32,
            rows: size.rows as u32,
            ..Default::default()
        })
        .await
        .map_err(|error| format!("could not {what} Agent shell: {error}"))
}

impl TerminalSessionHandle {
    pub async fn send_input(&self, data: &[u8]) -> Result<(), String> {
        match self {
            Self::AgentShell { sender } => {
                agent_message(
                    sender,
                    ShellAction::Input,
                    data.to_vec(),
                    TerminalSize { cols: 0, rows: 0 },
                    "write input to",
                )
                .await
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
                agent_message(sender, ShellAction::Resize, Vec::new(), size, "resize").await
            }
            Self::RusshShell { terminal } => terminal
                .resize(size.cols, size.rows)
                .await
                .map_err(|error| format!("could not resize SSH terminal: {error}")),
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
        if let TerminalSessionHandle::AgentShell { sender } = self.session {
            let _ = sender
                .lock()
                .await
                .send(ShellMessage {
                    action: ShellAction::Close as i32,
                    ..Default::default()
                })
                .await;
        }
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
