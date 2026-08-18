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
use remote::{spawn_remote_docker_terminal, spawn_remote_terminal};
pub use types::{
    DockerTerminalStart, ServerTerminalStart, SocketKey, TerminalInputPayload, TerminalKind,
    TerminalResize, TerminalSession,
};

#[derive(Debug)]
pub struct TerminalSocket {
    sessions: Arc<DashMap<SocketKey, TerminalSession>>,
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

    async fn terminate_session_for_key(session: TerminalSession, key_str: &str) {
        match session {
            TerminalSession::Pty { cancel, child, writer, .. } => {
                cancel.cancel();
                let mut w = writer.lock().await;
                let _ = w.shutdown().await;
                let mut c = child.lock().await;
                if let Some(pid) = c.id() {
                    let executor = crate::utils::exec::CommandExecutor::Local(crate::utils::exec::LocalExecutor::new());
                    let os = crate::utils::os::OsCli::new(&executor);
                    let _ = os.process_api().kill_pid(pid.to_string()).run().await;
                }
                let _ = c.kill().await;
                let _ = c.wait().await;
            }
            TerminalSession::Local { child, stdin, .. } => {
                let mut s = stdin.lock().await;
                let _ = s.shutdown().await;
                let mut c = child.lock().await;
                let _ = c.kill().await;
                let _ = c.wait().await;
            }
            TerminalSession::InMemorySsh { cancel, .. } => {
                cancel.cancel();
            }
            TerminalSession::DockerSocket { cancel, container, .. } => {
                cancel.cancel();
                if let Some(target) = container {
                    let k = key_str.to_string();
                    tokio::spawn(async move {
                        use os::exec::IntoCommand;
                        use sh_macros::sh;

                        let executor = crate::utils::exec::CommandExecutor::Local(crate::utils::exec::LocalExecutor::new());
                        let os = crate::utils::os::OsCli::new(&executor);

                        let proc_script_ir = sh!(
                            for p in ["/proc/[0-9]*"] {
                                if grep!("-q", word!["OPENOXIDE_SOCKET_ID=", dynamic!(k)], word!["$p", "/environ"]) {
                                    os.process_api().kill_pid("${p#/proc/}");
                                }
                            }
                        );

                        let bash_script = proc_script_ir
                            .iter()
                            .map(|step| step.to_bash())
                            .collect::<Vec<_>>()
                            .join("\n");

                        let _ = os.docker().containers().exec(target).run(["sh", "-c", &bash_script]).await;
                    });
                }
            }
        }

        let executor = crate::utils::exec::CommandExecutor::Local(crate::utils::exec::LocalExecutor::new());
        let os = crate::utils::os::OsCli::new(&executor);
        let _ = os.process_api()
            .pkill()
            .sigkill()
            .full_match(true)
            .env("OPENOXIDE_SOCKET_ID", key_str)
            .run()
            .await;
    }

    async fn stop_socket_session(&self, socket: &SocketRef) {
        let key = socket_key(socket);
        let key_str = key.to_string();
        if let Some((_, session)) = self.sessions.remove(&key) {
            Self::terminate_session_for_key(session, &key_str).await;
        }
    }

    fn bind_disconnect_cleanup(&self, socket: &SocketRef, key: SocketKey) {
        let sessions = self.sessions.clone();
        socket.on_disconnect(move |_socket: SocketRef, reason: DisconnectReason| {
            let sessions = sessions.clone();
            let key = key.clone();
            let key_str = key.to_string();
            async move {
                tracing::info!(%key, ?reason, "Terminal WebSocket disconnected; performing session cleanup");
                if let Some((_, session)) = sessions.remove(&key) {
                    Self::terminate_session_for_key(session, &key_str).await;
                }
            }
        });
    }

    #[on("docker:start")]
    async fn docker_start(&self, socket: SocketRef, Data(input): Data<DockerTerminalStart>) {
        self.stop_socket_session(&socket).await;
        self.bind_disconnect_cleanup(&socket, socket_key(&socket));

        if let Some(server_id) = input.server_id {
            spawn_remote_docker_terminal(socket, &self.sessions, self.db.as_ref(), server_id, input).await;
            return;
        }

        spawn_docker_terminal(socket, &self.sessions, input).await;
    }

    #[on("server:start")]
    async fn server_start(&self, socket: SocketRef, Data(mut input): Data<ServerTerminalStart>) {
        self.stop_socket_session(&socket).await;
        self.bind_disconnect_cleanup(&socket, socket_key(&socket));

        if input.shell.is_none() && input.command.is_some() {
            input.shell = input.command.clone();
        }

        if let Some(server_id) = input.server_id {
            spawn_remote_terminal(socket, &self.sessions, self.db.as_ref(), server_id, input).await;
            return;
        }

        let shell_cmd = input
            .shell
            .unwrap_or_else(|| std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".into()));
        let mut command = Command::new(shell_cmd);
        command
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        spawn_local_terminal(socket, &self.sessions, TerminalKind::Server, command).await;
    }

    #[on("input")]
    async fn input(&self, socket: SocketRef, Data(payload): Data<TerminalInputPayload>) {
        let key = socket_key(&socket);
        let data = match payload {
            TerminalInputPayload::Direct(s) => s,
            TerminalInputPayload::Object { data } => data,
        };

        let Some(session) = self.sessions.get(&key).map(|entry| entry.clone()) else {
            tracing::warn!("Input received for session {key} but session is not found");
            emit_error(&socket, "terminal session is not running");
            return;
        };

        match session {
            TerminalSession::Pty { writer, .. } => {
                let mut w = writer.lock().await;
                if let Err(error) = w.write_all(data.as_bytes()).await {
                    tracing::warn!("PTY write_all failed: {error}");
                    emit_error(&socket, format!("could not write PTY input: {error}"));
                } else {
                    let _ = w.flush().await;
                }
            }
            TerminalSession::Local { stdin, .. } => {
                let mut s = stdin.lock().await;
                if let Err(error) = s.write_all(data.as_bytes()).await {
                    tracing::warn!("Local terminal write_all failed: {error}");
                    emit_error(&socket, format!("could not write terminal input: {error}"));
                } else {
                    let _ = s.flush().await;
                }
            }
            TerminalSession::InMemorySsh { terminal, .. } => {
                let term = terminal.clone();
                let bytes = data.into_bytes();
                tokio::task::spawn_blocking(move || {
                    let _ = term.write(&bytes);
                });
            }
            TerminalSession::DockerSocket { writer, .. } => {
                let mut w = writer.lock().await;
                if let Err(error) = w.write_all(data.as_bytes()).await {
                    tracing::warn!("DockerSocket write_all failed: {error}");
                    emit_error(&socket, format!("could not write docker socket input: {error}"));
                } else {
                    let _ = w.flush().await;
                }
            }
        }
    }

    #[on("resize")]
    async fn resize(&self, socket: SocketRef, Data(payload): Data<TerminalResize>) {
        let key = socket_key(&socket);
        if let Some(session) = self.sessions.get(&key) {
            match session.value() {
                TerminalSession::Pty { writer, .. } => {
                    let w = writer.lock().await;
                    let _ = w.resize(payload.size());
                }
                TerminalSession::InMemorySsh { terminal, .. } => {
                    let term = terminal.clone();
                    let cols = payload.cols.unwrap_or(80);
                    let rows = payload.rows.unwrap_or(24);
                    tokio::task::spawn_blocking(move || {
                        let _ = term.resize(cols, rows);
                    });
                }
                TerminalSession::DockerSocket { socket_path, exec_id, .. } => {
                    let path = socket_path.clone();
                    let eid = exec_id.clone();
                    let cols = payload.cols.unwrap_or(80);
                    let rows = payload.rows.unwrap_or(24);
                    tokio::spawn(async move {
                        let _ = crate::utils::docker::handles::containers::socat::resize_container_exec(&path, &eid, cols, rows).await;
                    });
                }
                _ => {}
            }
        }
    }

    #[on("stop")]
    async fn stop(&self, socket: SocketRef) {
        self.stop_socket_session(&socket).await;
    }
}
