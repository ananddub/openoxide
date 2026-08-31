use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use crate::api::connect::generated::openoxide::api::v1::TerminalOutputStream;
use crate::services::observability::monitoring::proto::shell_message::Payload;
use crate::services::observability::monitoring::proto::ShellMessage;

use super::events::SessionEvents;
use super::registry::TerminalSessionRegistry;

/// Reader side of a freshly opened session, handed to the pump that forwards
/// PTY output into the event channel.
pub enum TerminalPump {
    Agent {
        stream: tonic::Streaming<ShellMessage>,
    },
    Russh {
        terminal: Arc<os::ssh::RusshTerminal>,
    },
}

impl TerminalPump {
    /// Start the backend-specific pump task that forwards PTY output into
    /// the event channel until the session ends, then deregisters it and
    /// emits a final exit event.
    pub fn spawn(
        self,
        events: SessionEvents,
        cancel: CancellationToken,
        registry: Arc<TerminalSessionRegistry>,
    ) {
        match self {
            Self::Agent { stream } => AgentPump {
                events,
                stream,
                cancel,
                registry,
            }
            .spawn(),
            Self::Russh { terminal } => RusshPump {
                events,
                terminal,
                cancel,
                registry,
            }
            .spawn(),
        }
    }
}

struct AgentPump {
    events: SessionEvents,
    stream: tonic::Streaming<ShellMessage>,
    cancel: CancellationToken,
    registry: Arc<TerminalSessionRegistry>,
}

impl AgentPump {
    fn spawn(self) {
        tokio::spawn(self.run());
    }

    async fn run(mut self) {
        let mut exit_code = Some(0);
        loop {
            tokio::select! {
                _ = self.cancel.cancelled() => break,
                result = self.stream.message() => match result {
                    Ok(Some(message)) => match message.payload {
                        Some(Payload::Stdout(output)) => {
                            if !self.events.emit_output(TerminalOutputStream::Stdout, output.data).await {
                                break;
                            }
                        }
                        Some(Payload::Stderr(output)) => {
                            if !self.events.emit_output(TerminalOutputStream::Stderr, output.data).await {
                                break;
                            }
                        }
                        Some(Payload::Exit(exit)) => {
                            exit_code = Some(exit.exit_code);
                            break;
                        }
                        Some(Payload::Error(error)) => {
                            let _ = self.events.emit_error(error.message).await;
                            exit_code = Some(-1);
                            break;
                        }
                        _ => {}
                    },
                    Ok(None) => break,
                    Err(error) => {
                        let _ = self
                            .events
                            .emit_error(format!("Agent shell stream failed: {error}"))
                            .await;
                        exit_code = Some(-1);
                        break;
                    }
                }
            }
        }
        self.registry.remove(self.events.session_id());
        let _ = self.events.emit_exit(exit_code).await;
    }
}

struct RusshPump {
    events: SessionEvents,
    terminal: Arc<os::ssh::RusshTerminal>,
    cancel: CancellationToken,
    registry: Arc<TerminalSessionRegistry>,
}

impl RusshPump {
    fn spawn(self) {
        tokio::spawn(self.run());
    }

    async fn run(self) {
        loop {
            tokio::select! {
                _ = self.cancel.cancelled() => break,
                output = self.terminal.read_next() => match output {
                    Some(bytes) => {
                        if !self
                            .events
                            .emit_output(TerminalOutputStream::Stdout, bytes)
                            .await
                        {
                            break;
                        }
                    }
                    None => break,
                }
            }
        }
        self.registry.remove(self.events.session_id());
        let _ = self.events.emit_exit(Some(0)).await;
    }
}
