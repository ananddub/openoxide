use std::sync::Arc;

use pty_process::{Command as PtyCommand, Size};
use socketioxide::extract::SocketRef;
use tokio::process::Command;
use tokio::sync::Mutex;

use super::helpers::{emit_error, emit_terminal_bytes, socket_key, spawn_pty_reader};
use super::types::{
    DockerTerminalStart, SessionMap, TerminalExit, TerminalSession, TerminalStarted,
};

pub async fn spawn_docker_terminal(
    socket: SocketRef,
    sessions: &SessionMap,
    input: DockerTerminalStart,
) {
    if input.server_id.is_some() {
        let msg = "\r\n\x1b[31m[Error] Remote docker terminal should be opened with server:start\x1b[0m\r\n";
        emit_terminal_bytes(&socket, "stdout", msg.as_bytes().to_vec());
        emit_error(&socket, "remote docker terminal should be opened with server:start");
        return;
    }

    let shell_req = input.shell.unwrap_or_else(|| "sh".into());
    let mut target_container = input.container.clone();

    let docker = crate::utils::docker::DockerCli::new_local();
    let mut container_found = false;

    if let Ok(containers) = docker.containers().ps().list().await {
        let search = input.container.to_lowercase();
        if let Some(matching) = containers.iter().find(|c| {
            let n = c.names.to_lowercase();
            n.contains(&search)
                || n.trim_start_matches('/').starts_with(&search)
                || n.contains(&format!("{}_", search))
                || n.contains(&format!("{}.", search))
        }) {
            target_container = matching.names.trim_start_matches('/').to_string();
            container_found = true;
        }
    }

    if !container_found && !input.container.is_empty() {
        target_container = input.container.clone();
    }

    let (pty, pts) = match pty_process::open() {
        Ok(res) => res,
        Err(error) => {
            let err_msg = format!("\r\n\x1b[31m[Error] Failed opening PTY system terminal: {error}\x1b[0m\r\n");
            emit_terminal_bytes(&socket, "stdout", err_msg.as_bytes().to_vec());
            emit_error(&socket, format!("could not open PTY: {error}"));
            return;
        }
    };
    let _ = pty.resize(Size::new(24, 80));

    let exec_args = docker
        .containers()
        .exec(&target_container)
        .interactive()
        .tty(true)
        .env("TERM", "xterm-256color")
        .workdir("/")
        .build_args([&shell_req]);

    let pty_cmd = PtyCommand::new("docker")
        .args(&exec_args)
        .env("TERM", "xterm-256color");

    let mut child = match pty_cmd.spawn(pts) {
        Ok(child) => child,
        Err(error) => {
            let err_msg = format!("\r\n\x1b[31m[Error] Failed to execute docker exec for container '{target_container}': {error}\x1b[0m\r\n");
            emit_terminal_bytes(&socket, "stdout", err_msg.as_bytes().to_vec());
            emit_error(&socket, format!("could not start docker exec terminal: {error}"));
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
    let shell_name = shell_req.clone();
    tokio::spawn(async move {
        let status = child.wait().await;
        sessions_clone.remove(&key);
        let code = status.ok().and_then(|s| s.code());
        if let Some(c) = code {
            if c != 0 {
                let err_msg = if c == 126 || c == 127 {
                    format!("\r\n\x1b[31m[Error] Shell '{shell_name}' is not installed in container '{container_name}'. Please switch shell dropdown to 'sh'.\x1b[0m\r\n")
                } else {
                    format!("\r\n\x1b[31m[Error] Container '{container_name}' shell process exited with code {c}. Check container status.\x1b[0m\r\n")
                };
                emit_terminal_bytes(&socket_clone, "stdout", err_msg.as_bytes().to_vec());
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
            let err_msg = format!("\r\n\x1b[31m[Error] Could not start local process: {error}\x1b[0m\r\n");
            emit_terminal_bytes(&socket, "stdout", err_msg.as_bytes().to_vec());
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
