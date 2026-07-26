pub mod helpers;
pub mod local;
pub mod remote;
pub mod types;

use std::sync::Arc;

use auto_di::singleton;
#[allow(unused_imports)]
use auto_socket::{auto_socket, on};
use dashmap::DashMap;
use socketioxide::{
    extract::{Data, SocketRef},
    socket::DisconnectReason,
};
use sqlx::SqlitePool;
use tokio::{io::AsyncWriteExt, process::Command};

use helpers::{emit_error, socket_key};
use local::{spawn_docker_terminal, spawn_local_terminal};
use remote::spawn_remote_terminal;
pub use types::{DockerTerminalStart, ServerTerminalStart, TerminalInput, TerminalResize, TerminalSession};

#[derive(Debug)]
pub struct TerminalSocket {
    sessions: Arc<DashMap<String, TerminalSession>>,
    db: Arc<SqlitePool>,
}

#[singleton]
#[auto_socket("/terminal")]
impl TerminalSocket {
    fn new(db: Arc<SqlitePool>) -> Self {
        Self {
            sessions: Arc::new(DashMap::new()),
            db,
        }
    }

    async fn stop_socket_session(&self, socket: &SocketRef) {
        let key = socket_key(socket);
        if let Some((_, session)) = self.sessions.remove(&key) {
            match session {
                TerminalSession::Pty { .. } => {}
                TerminalSession::Local { child, .. } => {
                    let _ = child.lock().await.kill().await;
                }
                TerminalSession::Remote { cancel, .. } => {
                    cancel.cancel();
                }
            }
        }
    }

    fn bind_disconnect_cleanup(&self, socket: &SocketRef, key: String) {
        let sessions = self.sessions.clone();
        socket.on_disconnect(move |_socket: SocketRef, _reason: DisconnectReason| {
            let sessions = sessions.clone();
            let key = key.clone();
            async move {
                if let Some((_, session)) = sessions.remove(&key) {
                    match session {
                        TerminalSession::Pty { .. } => {}
                        TerminalSession::Local { child, .. } => {
                            let _ = child.lock().await.kill().await;
                        }
                        TerminalSession::Remote { cancel, .. } => {
                            cancel.cancel();
                        }
                    }
                }
            }
        });
    }

    #[on("docker:start")]
    async fn docker_start(&self, socket: SocketRef, Data(input): Data<DockerTerminalStart>) {
        self.stop_socket_session(&socket).await;
        self.bind_disconnect_cleanup(&socket, socket_key(&socket));
        spawn_docker_terminal(socket, &self.sessions, input).await;
    }

    #[on("server:start")]
    async fn server_start(&self, socket: SocketRef, Data(input): Data<ServerTerminalStart>) {
        self.stop_socket_session(&socket).await;
        self.bind_disconnect_cleanup(&socket, socket_key(&socket));

        if let Some(server_id) = input.server_id {
            spawn_remote_terminal(socket, &self.sessions, self.db.as_ref(), server_id, input).await;
            return;
        }

        let shell = input
            .shell
            .unwrap_or_else(|| std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".into()));
        let mut command = Command::new(shell);
        command
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        spawn_local_terminal(socket, &self.sessions, "server", command).await;
    }

    #[on("input")]
    async fn input(&self, socket: SocketRef, Data(input): Data<TerminalInput>) {
        let key = socket_key(&socket);
        let Some(session) = self.sessions.get(&key).map(|entry| entry.clone()) else {
            emit_error(&socket, "terminal session is not running");
            return;
        };

        match session {
            TerminalSession::Pty { writer, .. } => {
                let mut w = writer.lock().await;
                if let Err(error) = w.write_all(input.data.as_bytes()).await {
                    tracing::warn!("PTY write_all failed: {error}");
                    emit_error(&socket, format!("could not write PTY input: {error}"));
                }
            }
            TerminalSession::Local { stdin, .. } => {
                let mut s = stdin.lock().await;
                if let Err(error) = s.write_all(input.data.as_bytes()).await {
                    tracing::warn!("Local terminal write_all failed: {error}");
                    emit_error(&socket, format!("could not write terminal input: {error}"));
                } else {
                    let _ = s.flush().await;
                }
            }
            TerminalSession::Remote { input: tx, .. } => {
                if tx.send(input.data.into_bytes()).await.is_err() {
                    emit_error(&socket, "remote terminal input channel is closed");
                }
            }
        }
    }

    #[on("resize")]
    async fn resize(&self, socket: SocketRef, Data(input): Data<TerminalResize>) {
        let key = socket_key(&socket);
        let Some(session) = self.sessions.get(&key).map(|entry| entry.clone()) else {
            emit_error(&socket, "terminal session is not running");
            return;
        };

        let cols = input.cols.unwrap_or(80);
        let rows = input.rows.unwrap_or(24);

        match session {
            TerminalSession::Pty { writer, .. } => {
                let w = writer.lock().await;
                if let Err(error) = w.resize(pty_process::Size::new(rows, cols)) {
                    tracing::warn!("PTY resize failed: {error}");
                    emit_error(&socket, format!("could not resize PTY: {error}"));
                }
            }
            TerminalSession::Remote { resize, .. } => {
                if resize.send((rows, cols)).await.is_err() {
                    tracing::warn!("Remote resize channel is closed");
                    emit_error(&socket, "remote terminal resize channel is closed");
                }
            }
            TerminalSession::Local { .. } => {}
        }
    }

    #[on("stop")]
    async fn stop(&self, socket: SocketRef) {
        self.stop_socket_session(&socket).await;
    }
}
