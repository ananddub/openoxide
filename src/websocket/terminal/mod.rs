pub mod helpers;
pub mod remote;
pub mod types;

use std::sync::Arc;

use auto_di::singleton;
#[allow(unused_imports)]
use auto_socket::{auto_socket, on};
use dashmap::DashMap;
use helpers::{emit_error, socket_key};
use remote::spawn_remote_docker_terminal_api;
use socketioxide::{
    extract::{Data, SocketRef},
    socket::DisconnectReason,
};
pub use types::{
    DockerTerminalStart, ServerTerminalStart, SocketKey, TerminalInputPayload, TerminalKind,
    TerminalResize, TerminalSession,
};

#[derive(Debug)]
pub struct TerminalSocket {
    sessions: Arc<DashMap<SocketKey, TerminalSession>>,
    pending_starts: Arc<DashMap<SocketKey, tokio_util::sync::CancellationToken>>,
}

#[singleton]
#[auto_socket("/terminal")]
impl TerminalSocket {
    fn new() -> Self {
        Self {
            sessions: Arc::new(DashMap::new()),
            pending_starts: Arc::new(DashMap::new()),
        }
    }

    async fn terminate_session(session: TerminalSession) {
        match session {
            TerminalSession::AgentShell { sender, cancel, .. } => {
                cancel.cancel();
                let sender = sender.lock().await;
                let _ = sender
                    .send(crate::services::monitoring::proto::ShellMessage {
                        action: crate::services::monitoring::proto::ShellAction::Close as i32,
                        ..Default::default()
                    })
                    .await;
            }
            TerminalSession::RusshShell { cancel, .. } => {
                cancel.cancel();
            }
        }
    }

    async fn stop_socket_session(&self, socket: &SocketRef) {
        let key = socket_key(socket);
        if let Some((_, token)) = self.pending_starts.remove(&key) {
            token.cancel();
        }
        if let Some((_, session)) = self.sessions.remove(&key) {
            Self::terminate_session(session).await;
        }
    }

    fn prepare_start_token(&self, socket: &SocketRef) -> tokio_util::sync::CancellationToken {
        let key = socket_key(socket);
        if let Some((_, token)) = self.pending_starts.remove(&key) {
            token.cancel();
        }
        let token = tokio_util::sync::CancellationToken::new();
        self.pending_starts.insert(key, token.clone());
        token
    }

    fn bind_disconnect_cleanup(&self, socket: &SocketRef, key: SocketKey) {
        let sessions = self.sessions.clone();
        socket.on_disconnect(move |_socket: SocketRef, reason: DisconnectReason| {
            let sessions = sessions.clone();
            let key = key.clone();
            async move {
                tracing::info!(%key, ?reason, "Terminal WebSocket disconnected; performing session cleanup");
                if let Some((_, session)) = sessions.remove(&key) {
                    Self::terminate_session(session).await;
                }
            }
        });
    }

    #[on("docker:start")]
    async fn docker_start(&self, socket: SocketRef, Data(input): Data<DockerTerminalStart>) {
        self.stop_socket_session(&socket).await;
        let start_token = self.prepare_start_token(&socket);
        self.bind_disconnect_cleanup(&socket, socket_key(&socket));

        let server_id =
            match crate::services::compose::remote::agent_server_id(input.server_id).await {
                Ok(server_id) => server_id,
                Err(error) => {
                    emit_error(&socket, error);
                    return;
                }
            };
        spawn_remote_docker_terminal_api(socket, &self.sessions, server_id, input, start_token)
            .await;
    }

    #[on("server:start")]
    async fn server_start(&self, socket: SocketRef, Data(mut input): Data<ServerTerminalStart>) {
        self.stop_socket_session(&socket).await;
        let start_token = self.prepare_start_token(&socket);
        self.bind_disconnect_cleanup(&socket, socket_key(&socket));

        if input.shell.is_none() && input.command.is_some() {
            input.shell = input.command.clone();
        }

        let Some(server_id) = input.server_id else {
            emit_error(
                &socket,
                "A remote server is required for SSH terminal".to_owned(),
            );
            return;
        };
        remote::spawn_russh_server_terminal(socket, &self.sessions, server_id, input, start_token)
            .await;
    }

    #[on("input")]
    async fn input(&self, socket: SocketRef, Data(payload): Data<TerminalInputPayload>) {
        let key = socket_key(&socket);
        let data = match payload {
            TerminalInputPayload::Direct(s) => s,
            TerminalInputPayload::Object { data } => data,
        };

        let Some(session) = self.sessions.get(&key).map(|entry| entry.clone()) else {
            // The browser can flush a queued input event after startup failed
            // (for example while an Agent reinstall is being queued). The
            // startup error is already emitted; do not mask it with a second
            // misleading terminal-state error.
            tracing::debug!("Input received for closed terminal session {key}");
            return;
        };

        match session {
            TerminalSession::AgentShell { sender, .. } => {
                let sender = sender.lock().await;
                if let Err(error) = sender
                    .send(crate::services::monitoring::proto::ShellMessage {
                        action: crate::services::monitoring::proto::ShellAction::Input as i32,
                        data: data.into_bytes(),
                        ..Default::default()
                    })
                    .await
                {
                    emit_error(
                        &socket,
                        format!("could not write Agent shell input: {error}"),
                    );
                }
            }
            TerminalSession::RusshShell { terminal, .. } => {
                if let Err(error) = terminal.write(data.as_bytes()).await {
                    emit_error(
                        &socket,
                        format!("could not write SSH terminal input: {error}"),
                    );
                }
            }
        }
    }

    #[on("resize")]
    async fn resize(&self, socket: SocketRef, Data(payload): Data<TerminalResize>) {
        let key = socket_key(&socket);
        if let Some(session) = self.sessions.get(&key) {
            match session.value() {
                TerminalSession::AgentShell { sender, .. } => {
                    let sender = sender.clone();
                    let size = payload.size();
                    tokio::spawn(async move {
                        let sender = sender.lock().await;
                        let _ = sender
                            .send(crate::services::monitoring::proto::ShellMessage {
                                action: crate::services::monitoring::proto::ShellAction::Resize
                                    as i32,
                                cols: size.cols as u32,
                                rows: size.rows as u32,
                                ..Default::default()
                            })
                            .await;
                    });
                }
                TerminalSession::RusshShell { terminal, .. } => {
                    let terminal = terminal.clone();
                    let size = payload.size();
                    tokio::spawn(async move {
                        let _ = terminal.resize(size.cols, size.rows).await;
                    });
                }
            }
        }
    }

    #[on("stop")]
    async fn stop(&self, socket: SocketRef) {
        self.stop_socket_session(&socket).await;
    }
}
