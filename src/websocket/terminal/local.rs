use std::sync::Arc;

use pty_process::{Command as PtyCommand, Size};
use socketioxide::extract::SocketRef;
use tokio::process::Command;
use tokio::sync::Mutex;

use super::helpers::{emit_error, socket_key, spawn_pty_reader};
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

    let shell_req = input.shell.unwrap_or_else(|| "sh".into());
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

    // Universal shell fallback: try requested shell first, fallback to sh if bash is missing in Alpine/Nginx
    let exec_cmd_args: Vec<String> = if shell_req == "bash" {
        vec!["sh".into(), "-c".into(), "exec bash 2>/dev/null || exec sh".into()]
    } else {
        vec![shell_req.clone()]
    };

    let cmd = if use_docker {
        let exec_args = docker
            .containers()
            .exec(&target_container)
            .interactive()
            .tty(true)
            .env("TERM", "xterm-256color")
            .workdir("/")
            .build_args(&exec_cmd_args);
        PtyCommand::new("docker")
            .args(&exec_args)
            .env("TERM", "xterm-256color")
    } else {
        PtyCommand::new(&shell_req).env("TERM", "xterm-256color")
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
    let container_name = target_container.clone();
    tokio::spawn(async move {
        let status = child.wait().await;
        sessions_clone.remove(&key);
        let code = status.ok().and_then(|s| s.code());
        if let Some(c) = code {
            if c != 0 {
                emit_error(
                    &socket_clone,
                    format!("Container '{container_name}' terminal exited with code {c}. Check if container is running."),
                );
            }
        }
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

    let stdin = child
        .stdin
        .take()
        .expect("stdin was configured with piped()");
    let stdout = child
        .stdout
        .take()
        .expect("stdout was configured with piped()");
    let stderr = child
        .stderr
        .take()
        .expect("stderr was configured with piped()");

    sessions.insert(
        key.clone(),
        TerminalSession::Local {
            stdin: Arc::new(Mutex::new(stdin)),
            child: Arc::new(Mutex::new(child)),
        },
    );

    let _ = socket.emit("started", &TerminalStarted { kind });

    super::helpers::spawn_output_task(socket.clone(), "stdout", stdout);
    super::helpers::spawn_output_task(socket.clone(), "stderr", stderr);
}
