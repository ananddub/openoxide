use std::sync::Arc;
use std::task::{Context, Poll};

use tokio::sync::mpsc;

use crate::api::connect::generated::openoxide::api::v1::TerminalEvent;

use super::registry::TerminalSessionRegistry;

/// A freshly opened terminal session: its id, the event stream the client
/// consumes, and the registry it deregisters from. Dropping it (client
/// disconnect or natural end) tears the session down: the registry entry is
/// removed, the pumps are cancelled and the Agent channel receives an
/// explicit close message before its sender is dropped.
pub struct TerminalSession {
    session_id: String,
    events: mpsc::Receiver<TerminalEvent>,
    registry: Arc<TerminalSessionRegistry>,
}

impl TerminalSession {
    pub(super) fn new(
        session_id: String,
        events: mpsc::Receiver<TerminalEvent>,
        registry: Arc<TerminalSessionRegistry>,
    ) -> Self {
        Self {
            session_id,
            events,
            registry,
        }
    }

    /// Poll the next terminal event from the session's event channel.
    pub fn poll_next(&mut self, cx: &mut Context<'_>) -> Poll<Option<TerminalEvent>> {
        self.events.poll_recv(cx)
    }
}

impl Drop for TerminalSession {
    fn drop(&mut self) {
        let session_id = std::mem::take(&mut self.session_id);
        if let Ok(handle) = tokio::runtime::Handle::try_current() {
            let registry = Arc::clone(&self.registry);
            let _ = handle.spawn(async move {
                registry.remove_and_terminate(&session_id).await;
            });
        } else {
            // No runtime (process shutdown): dropping the entry still closes
            // the Agent request channel and the SSH terminal.
            self.registry.remove(&session_id);
        }
    }
}

/// Failure surfaced by [`TerminalService::command`]. The transport layer maps
/// this onto ConnectRPC error codes.
pub enum TerminalCommandError {
    PermissionDenied,
    Failed(String),
}
