use std::io::{Read, Write};
use std::sync::Arc;

use auto_di::singleton;
#[allow(unused_imports)]
use auto_socket::{auto_socket, on};
use dashmap::DashMap;
use portable_pty::{CommandBuilder, MasterPty, PtySize, native_pty_system};
use serde::{Deserialize, Serialize};
use socketioxide::{
    extract::{Data, SocketRef},
    socket::DisconnectReason,
};
use sqlx::SqlitePool;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    process::{Child, ChildStdin, Command},
    sync::{Mutex, mpsc},
};
use tokio_util::sync::CancellationToken;

use crate::utils::exec::ExecStreamEvent;

#[derive(Debug, Deserialize)]
pub struct DockerTerminalStart {
    pub container: String,
    pub shell: Option<String>,
    pub server_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ServerTerminalStart {
    pub shell: Option<String>,
    pub server_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct TerminalInput {
    pub data: String,
}

#[derive(Debug, Deserialize)]
pub struct TerminalResize {
    pub cols: Option<u16>,
    pub rows: Option<u16>,
}

#[derive(Debug, Serialize)]
struct TerminalOutput<'a> {
    stream: &'a str,
    data: String,
}

#[derive(Debug, Serialize)]
struct TerminalStarted<'a> {
    kind: &'a str,
}

#[derive(Debug, Serialize)]
struct TerminalError {
    message: String,
}

#[derive(Debug, Serialize)]
struct TerminalExit {
    code: Option<i32>,
}

#[derive(Clone)]
enum TerminalSession {
    Pty {
        writer: Arc<Mutex<Box<dyn Write + Send>>>,
        master: Arc<Mutex<Box<dyn MasterPty + Send>>>,
    },
    Local {
        stdin: Arc<Mutex<ChildStdin>>,
        child: Arc<Mutex<Child>>,
    },
    Remote {
        input: mpsc::Sender<Vec<u8>>,
        resize: mpsc::Sender<(u16, u16)>,
        cancel: CancellationToken,
    },
}

impl std::fmt::Debug for TerminalSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pty { .. } => write!(f, "TerminalSession::Pty"),
            Self::Local { .. } => write!(f, "TerminalSession::Local"),
            Self::Remote { .. } => write!(f, "TerminalSession::Remote"),
        }
    }
}

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

    #[on("docker:start")]
    async fn docker_start(&self, socket: SocketRef, Data(input): Data<DockerTerminalStart>) {
        if input.server_id.is_some() {
            emit_error(
                &socket,
                "remote docker terminal should be opened with server:start and docker command inside the remote shell",
            );
            return;
        }

        let shell = input.shell.unwrap_or_else(|| "sh".into());
        let mut target_container = input.container.clone();

        let docker = crate::utils::docker::DockerCli::new_local();
        if let Ok(containers) = docker
            .containers()
            .ps()
            .filter(crate::utils::docker::query::ContainerFilter::Name(input.container.clone()))
            .list()
            .await
        {
            if let Some(first) = containers.first() {
                target_container = first.names.trim_start_matches('/').to_string();
            }
        }

        // Spawn PTY process for real interactive shell
        let pty_system = native_pty_system();
        let pair = match pty_system.openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        }) {
            Ok(pair) => pair,
            Err(error) => {
                emit_error(&socket, format!("could not open PTY: {error}"));
                return;
            }
        };

        let exec_args = docker
            .containers()
            .exec(&target_container)
            .interactive()
            .tty(true)
            .workdir("/")
            .build_args([&shell]);

        let mut cmd = CommandBuilder::new("docker");
        cmd.args(&exec_args);

        let child = match pair.slave.spawn_command(cmd) {
            Ok(child) => child,
            Err(error) => {
                emit_error(&socket, format!("could not start docker terminal: {error}"));
                return;
            }
        };

        let reader = match pair.master.try_clone_reader() {
            Ok(r) => r,
            Err(error) => {
                emit_error(&socket, format!("could not clone PTY reader: {error}"));
                return;
            }
        };

        let writer = match pair.master.take_writer() {
            Ok(w) => w,
            Err(error) => {
                emit_error(&socket, format!("could not take PTY writer: {error}"));
                return;
            }
        };

        let key = socket_key(&socket);
        self.stop_socket(&socket).await;
        self.bind_disconnect_cleanup(&socket, key.clone());

        self.sessions.insert(
            key.clone(),
            TerminalSession::Pty {
                writer: Arc::new(Mutex::new(writer)),
                master: Arc::new(Mutex::new(pair.master)),
            },
        );

        let _ = socket.emit("started", &TerminalStarted { kind: "docker" });

        let output_socket = socket.clone();
        std::thread::spawn(move || {
            let mut reader = reader;
            let mut buffer = [0u8; 8192];
            loop {
                match reader.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(n) => emit_terminal_bytes(&output_socket, "stdout", buffer[..n].to_vec()),
                    Err(_) => break,
                }
            }
            let _ = output_socket.emit("exit", &TerminalExit { code: Some(0) });
        });

        let mut child = child;
        let sessions = self.sessions.clone();
        tokio::spawn(async move {
            let _ = tokio::task::spawn_blocking(move || child.wait()).await;
            sessions.remove(&key);
        });
    }

    #[on("server:start")]
    async fn server_start(&self, socket: SocketRef, Data(input): Data<ServerTerminalStart>) {
        if let Some(server_id) = input.server_id {
            self.spawn_remote_terminal(socket, server_id, input).await;
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

        self.spawn_local_terminal(socket, "server", command).await;
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
                if let Err(error) = w.write_all(input.data.as_bytes()) {
                    emit_error(&socket, format!("could not write PTY input: {error}"));
                } else {
                    let _ = w.flush();
                }
            }
            TerminalSession::Local { stdin, .. } => {
                if let Err(error) = stdin.lock().await.write_all(input.data.as_bytes()).await {
                    emit_error(&socket, format!("could not write terminal input: {error}"));
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

        match session {
            TerminalSession::Pty { master, .. } => {
                let cols = input.cols.unwrap_or(80);
                let rows = input.rows.unwrap_or(24);
                let _ = master.lock().await.resize(PtySize {
                    rows,
                    cols,
                    pixel_width: 0,
                    pixel_height: 0,
                });
            }
            TerminalSession::Remote { resize, .. } => {
                let cols = input.cols.unwrap_or(80);
                let rows = input.rows.unwrap_or(24);
                if resize.send((cols, rows)).await.is_err() {
                    emit_error(&socket, "remote terminal resize channel is closed");
                }
            }
            TerminalSession::Local { .. } => {}
        }
    }

    #[on("stop")]
    async fn stop(&self, socket: SocketRef) {
        self.stop_socket(&socket).await;
    }

    async fn spawn_local_terminal(
        &self,
        socket: SocketRef,
        kind: &'static str,
        mut command: Command,
    ) {
        self.stop_socket(&socket).await;

        let key = socket_key(&socket);
        self.bind_disconnect_cleanup(&socket, key.clone());

        let mut child = match command.spawn() {
            Ok(child) => child,
            Err(error) => {
                emit_error(&socket, format!("could not start terminal: {error}"));
                return;
            }
        };

        let Some(stdin) = child.stdin.take() else {
            emit_error(&socket, "terminal stdin is unavailable");
            let _ = child.kill().await;
            return;
        };

        let stdout = child.stdout.take();
        let stderr = child.stderr.take();
        let child = Arc::new(Mutex::new(child));
        self.sessions.insert(
            key.clone(),
            TerminalSession::Local {
                stdin: Arc::new(Mutex::new(stdin)),
                child: child.clone(),
            },
        );

        let _ = socket.emit("started", &TerminalStarted { kind });
        if let Some(stdout) = stdout {
            spawn_output_task(socket.clone(), "stdout", stdout);
        }
        if let Some(stderr) = stderr {
            spawn_output_task(socket.clone(), "stderr", stderr);
        }

        let sessions = self.sessions.clone();
        tokio::spawn(async move {
            let status = child.lock().await.wait().await;
            sessions.remove(&key);
            match status {
                Ok(status) => {
                    let _ = socket.emit(
                        "exit",
                        &TerminalExit {
                            code: status.code(),
                        },
                    );
                }
                Err(error) => emit_error(&socket, format!("terminal wait failed: {error}")),
            }
        });
    }

    async fn spawn_remote_terminal(
        &self,
        socket: SocketRef,
        server_id: i64,
        input: ServerTerminalStart,
    ) {
        self.stop_socket(&socket).await;

        let key = socket_key(&socket);
        self.bind_disconnect_cleanup(&socket, key.clone());
        let (output_tx, mut output_rx) = mpsc::channel::<ExecStreamEvent>(256);

        let executor =
            match crate::services::compose::remote::remote_executor(self.db.as_ref(), server_id)
                .await
            {
                Ok(executor) => executor,
                Err(error) => {
                    emit_error(
                        &socket,
                        format!("could not create remote executor: {error}"),
                    );
                    return;
                }
            };

        let terminal = match executor
            .open_terminal(
                output_tx,
                input.shell.unwrap_or_else(|| "xterm-256color".into()),
                80,
                24,
            )
            .await
        {
            Ok(terminal) => terminal,
            Err(error) => {
                emit_error(&socket, format!("could not start remote terminal: {error}"));
                return;
            }
        };

        self.sessions.insert(
            key.clone(),
            TerminalSession::Remote {
                input: terminal.input.clone(),
                resize: terminal.resize.clone(),
                cancel: terminal.cancel.clone(),
            },
        );

        let _ = socket.emit(
            "started",
            &TerminalStarted {
                kind: "remote-server",
            },
        );

        let output_socket = socket.clone();
        tokio::spawn(async move {
            while let Some(event) = output_rx.recv().await {
                match event {
                    ExecStreamEvent::Stdout(bytes) => {
                        emit_terminal_bytes(&output_socket, "stdout", bytes)
                    }
                    ExecStreamEvent::Stderr(bytes) => {
                        emit_terminal_bytes(&output_socket, "stderr", bytes)
                    }
                }
            }
        });

        let sessions = self.sessions.clone();
        tokio::spawn(async move {
            let result = terminal.wait().await;
            sessions.remove(&key);
            match result {
                Ok(()) => {
                    let _ = socket.emit("exit", &TerminalExit { code: None });
                }
                Err(error) => emit_error(&socket, format!("remote terminal failed: {error}")),
            }
        });
    }

    fn bind_disconnect_cleanup(&self, socket: &SocketRef, key: String) {
        let sessions = self.sessions.clone();
        socket.on_disconnect(move |_socket: SocketRef, _reason: DisconnectReason| {
            let sessions = sessions.clone();
            let key = key.clone();
            async move {
                if let Some((_, session)) = sessions.remove(&key) {
                    stop_session(session).await;
                }
            }
        });
    }

    async fn stop_socket(&self, socket: &SocketRef) {
        let key = socket_key(socket);
        let Some((_, session)) = self.sessions.remove(&key) else {
            return;
        };

        stop_session(session).await;
    }
}

async fn stop_session(session: TerminalSession) {
    match session {
        TerminalSession::Pty { .. } => {}
        TerminalSession::Local { child, .. } => {
            if let Err(error) = child.lock().await.kill().await {
                tracing::warn!(error = %error, "could not kill terminal child process");
            }
        }
        TerminalSession::Remote { cancel, .. } => {
            cancel.cancel();
        }
    }
}

fn emit_terminal_bytes(socket: &SocketRef, stream: &'static str, bytes: Vec<u8>) {
    let data = String::from_utf8_lossy(&bytes).into_owned();
    let _ = socket.emit("output", &TerminalOutput { stream, data });
}

fn emit_error(socket: &SocketRef, message: impl Into<String>) {
    let _ = socket.emit(
        "error",
        &TerminalError {
            message: message.into(),
        },
    );
}

fn socket_key(socket: &SocketRef) -> String {
    socket.id.to_string()
}

fn spawn_output_task(
    socket: SocketRef,
    stream: &'static str,
    mut reader: impl tokio::io::AsyncRead + Unpin + Send + 'static,
) {
    tokio::spawn(async move {
        let mut buffer = vec![0; 8192];
        loop {
            match reader.read(&mut buffer).await {
                Ok(0) => return,
                Ok(n) => emit_terminal_bytes(&socket, stream, buffer[..n].to_vec()),
                Err(error) => {
                    emit_error(&socket, format!("terminal read failed: {error}"));
                    return;
                }
            }
        }
    });
}
