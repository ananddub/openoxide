use std::sync::Arc;

use pty_process::{Command as PtyCommand, Size};
use socketioxide::extract::SocketRef;
use tokio::sync::Mutex;

use super::helpers::{emit_error, emit_terminal_bytes, socket_key, spawn_pty_reader};
use super::types::{
    ServerTerminalStart, SessionMap, TerminalExit, TerminalSession, TerminalStarted,
};

pub async fn spawn_remote_terminal(
    socket: SocketRef,
    sessions: &SessionMap,
    db: &sqlx::SqlitePool,
    server_id: i64,
    input: ServerTerminalStart,
) {
    let key = socket_key(&socket);

    let executor = match crate::services::compose::remote::remote_executor(db, server_id).await {
        Ok(executor) => executor,
        Err(error) => {
            let err_msg = format!("\r\n\x1b[31m[Error] Could not create remote SSH executor: {error}\x1b[0m\r\n");
            emit_terminal_bytes(&socket, "stdout", err_msg.as_bytes().to_vec());
            emit_error(
                &socket,
                format!("could not create remote executor: {error}"),
            );
            return;
        }
    };

    let shell_req = input.shell.unwrap_or_else(|| "bash".into());
    let builder = crate::utils::ssh::SshBuilder::new(
        executor.host().to_string(),
        executor.username().to_string(),
        executor.auth().clone(),
        executor.host_key().clone(),
    )
    .port(executor.port())
    .disable_multiplexing()
    .quiet(true)
    .tty(crate::utils::ssh::TtyMode::ForceTty);

    // The builder loads a KeyPair into a private ssh-agent itself, so the key
    // never reaches disk. The session must outlive the command.
    let (mut args, agent_session, temp_askpass, agent_socket) = match builder.build_args().await {
        Ok(res) => res,
        Err(error) => {
            let err_msg = format!("\r\n\x1b[31m[Error] Could not build SSH authentication arguments: {error}\x1b[0m\r\n");
            emit_terminal_bytes(&socket, "stdout", err_msg.as_bytes().to_vec());
            emit_error(&socket, format!("could not build SSH args: {error}"));
            return;
        }
    };

    let remote_cmd = if shell_req == "bash" {
        "stty -echoctl 2>/dev/null; export TERM=xterm-256color 2>/dev/null; exec bash 2>/dev/null || exec sh".to_string()
    } else {
        format!("stty -echoctl 2>/dev/null; export TERM=xterm-256color 2>/dev/null; exec {shell_req}")
    };
    args.push(remote_cmd);

    let (pty, pts) = match pty_process::open() {
        Ok(res) => res,
        Err(error) => {
            let err_msg = format!("\r\n\x1b[31m[Error] Could not open PTY for SSH terminal: {error}\x1b[0m\r\n");
            emit_terminal_bytes(&socket, "stdout", err_msg.as_bytes().to_vec());
            emit_error(&socket, format!("could not open PTY: {error}"));
            return;
        }
    };
    let _ = pty.resize(Size::new(24, 80));

    let pty_cmd = PtyCommand::new("ssh")
        .args(&args)
        .env("TERM", "xterm-256color");
    let mut cmd = pty_cmd;
    if let Some(socket_path) = agent_socket {
        cmd = cmd.env("SSH_AUTH_SOCK", socket_path);
    }
    if let Some(ref askpass) = temp_askpass {
        cmd = cmd
            .env("SSH_ASKPASS", askpass.as_os_str())
            .env("SSH_ASKPASS_REQUIRE", "force")
            .env("DISPLAY", ":0");
    }

    let mut child = match cmd.spawn(pts) {
        Ok(child) => child,
        Err(error) => {
            let err_msg = format!("\r\n\x1b[31m[Error] Could not spawn SSH terminal process: {error}\x1b[0m\r\n");
            emit_terminal_bytes(&socket, "stdout", err_msg.as_bytes().to_vec());
            emit_error(&socket, format!("could not start SSH terminal: {error}"));
            return;
        }
    };

    let (reader, writer) = pty.into_split();

    sessions.insert(
        key.clone(),
        TerminalSession::Pty {
            writer: Arc::new(Mutex::new(writer)),
        },
    );

    let _ = socket.emit(
        "started",
        &TerminalStarted {
            kind: "remote-server",
        },
    );

    spawn_pty_reader(socket.clone(), reader);

    let sessions_clone = sessions.clone();
    let socket_clone = socket.clone();
    let server_host = executor.host().to_string();
    tokio::spawn(async move {
        let _keep_alive_agent = agent_session;
        let _keep_alive_askpass = temp_askpass;
        let status = child.wait().await;
        sessions_clone.remove(&key);
        let code = status.ok().and_then(|s| s.code());
        if let Some(c) = code {
            if c != 0 {
                let err_msg = format!("\r\n\x1b[31m[Error] SSH session to server '{server_host}' closed with exit code {c}.\x1b[0m\r\n");
                emit_terminal_bytes(&socket_clone, "stdout", err_msg.as_bytes().to_vec());
            }
        }
        let _ = socket_clone.emit("exit", &TerminalExit { code });
    });
}
