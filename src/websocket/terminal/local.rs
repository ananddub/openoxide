use std::io::Read;
use std::sync::Arc;

use portable_pty::{CommandBuilder, PtySize, native_pty_system};
use socketioxide::extract::SocketRef;
use tokio::process::Command;
use tokio::sync::Mutex;

use super::helpers::{emit_error, emit_terminal_bytes, socket_key, spawn_output_task};
use super::types::{DockerTerminalStart, SessionMap, TerminalExit, TerminalSession, TerminalStarted};

pub async fn spawn_docker_terminal(
    socket: SocketRef,
    sessions: &SessionMap,
    input: DockerTerminalStart,
) {
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
    sessions.insert(
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
    let sessions_clone = sessions.clone();
    tokio::spawn(async move {
        let _ = tokio::task::spawn_blocking(move || child.wait()).await;
        sessions_clone.remove(&key);
    });
}

pub async fn spawn_local_terminal(
    socket: SocketRef,
    sessions: &SessionMap,
    kind: &'static str,
    mut command: Command,
) {
    let key = socket_key(&socket);

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
    sessions.insert(
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

    let sessions_clone = sessions.clone();
    let socket_clone = socket.clone();
    tokio::spawn(async move {
        let status = child.lock().await.wait().await;
        sessions_clone.remove(&key);
        match status {
            Ok(status) => {
                let _ = socket_clone.emit(
                    "exit",
                    &TerminalExit {
                        code: status.code(),
                    },
                );
            }
            Err(error) => emit_error(&socket_clone, format!("terminal wait failed: {error}")),
        }
    });
}
