use std::sync::Arc;

use pty_process::{Command as PtyCommand, Size};
use socketioxide::extract::SocketRef;
use tokio::process::Command;
use tokio::sync::Mutex;

use super::helpers::{emit_error, socket_key, spawn_output_task, spawn_pty_reader};
use super::types::{
    DockerTerminalStart, SessionMap, TerminalExit, TerminalSession, TerminalStarted,
};

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
    let mut use_docker = false;
    if let Ok(containers) = docker.containers().ps().list().await {
        if !containers.is_empty() {
            use_docker = true;
            let search = input.container.to_lowercase();
            if let Some(matching) = containers.iter().find(|c| {
                let n = c.names.to_lowercase();
                n.contains(&search)
                    || n.contains(&format!("{}_", search))
                    || n.contains(&format!("{}.", search))
            }) {
                target_container = matching.names.trim_start_matches('/').to_string();
            } else {
                emit_error(
                    &socket,
                    format!(
                        "container for service '{}' is not currently running",
                        input.container
                    ),
                );
                return;
            }
        }
    }

    let (pty, pts) = match pty_process::open() {
        Ok(res) => res,
        Err(error) => {
            emit_error(&socket, format!("could not open PTY: {error}"));
            return;
        }
    };
    let _ = pty.resize(Size::new(24, 80));

    let cmd = if use_docker {
        let exec_args = docker
            .containers()
            .exec(&target_container)
            .interactive()
            .tty(true)
            .env("TERM", "xterm-256color")
            .workdir("/")
            .build_args([&shell]);
        PtyCommand::new("docker")
            .args(&exec_args)
            .env("TERM", "xterm-256color")
    } else {
        PtyCommand::new(&shell).env("TERM", "xterm-256color")
    };

    let mut child = match cmd.spawn(pts) {
        Ok(child) => child,
        Err(error) => {
            emit_error(&socket, format!("could not start terminal: {error}"));
            return;
        }
    };

    let (reader, writer) = pty.into_split();

    let key = socket_key(&socket);
    sessions.insert(
        key.clone(),
        TerminalSession::Pty {
            writer: Arc::new(Mutex::new(writer)),
        },
    );

    let _ = socket.emit("started", &TerminalStarted { kind: "docker" });

    spawn_pty_reader(socket.clone(), reader);

    let sessions_clone = sessions.clone();
    let socket_clone = socket.clone();
    tokio::spawn(async move {
        let status = child.wait().await;
        sessions_clone.remove(&key);
        let code = status.ok().and_then(|s| s.code());
        let _ = socket_clone.emit("exit", &TerminalExit { code });
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
