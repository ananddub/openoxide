use std::sync::Arc;

use pty_process::{Command as PtyCommand, Size};
use socketioxide::extract::SocketRef;
use tokio::sync::Mutex;

use super::helpers::{emit_error, emit_terminal_bytes, next_session_id, socket_key, spawn_pty_reader};
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
            tracing::error!(server_id, %error, "remote_executor failed in spawn_remote_terminal");
            let err_msg = format!("\r\n\x1b[31m[Error] Could not create remote SSH executor: {error}\x1b[0m\r\n");
            emit_terminal_bytes(&socket, "stdout", err_msg.as_bytes().to_vec());
            emit_error(
                &socket,
                format!("could not create remote executor: {error}"),
            );
            return;
        }
    };

    let actual_host = executor.host().to_string();

    let shell_req = input.shell.unwrap_or_else(|| "bash".into());
    let builder = crate::utils::ssh::SshBuilder::new(
        actual_host.clone(),
        executor.username().to_string(),
        executor.auth().clone(),
        executor.host_key().clone(),
    )
    .port(executor.port())
    .connect_timeout(5)
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

    // Build the remote shell command.
    // bash: single command string — sshd wraps it in `/bin/sh -c "..."` which then execs bash.
    //   TERM + COLORTERM enable 256-color/truecolor.
    //   PROMPT_COMMAND sets PS1 *after* ~/.bashrc runs so it always wins.
    //   Colorful prompt: green user@host, blue cwd, then $ / #
    // sh: plain `sh -i` — dash does not support color PS1 escape sequences reliably.
    if shell_req == "sh" {
        args.push("sh".to_string());
        args.push("-i".to_string());
    } else {
        let ps1 = r"\[\e[0;32m\]\u@\h\[\e[0m\]:\[\e[0;34m\]\w\[\e[0m\]\$ ";
        args.push(format!(
            "TERM=xterm-256color COLORTERM=truecolor \
             PROMPT_COMMAND='PS1=\"{ps1}\"' \
             bash -i"
        ));
    }
    let shell_bin = if shell_req == "sh" { "sh" } else { "bash" };

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

    let child = match cmd.spawn(pts) {
        Ok(child) => child,
        Err(error) => {
            let err_msg = format!("\r\n\x1b[31m[Error] Could not spawn SSH terminal process: {error}\x1b[0m\r\n");
            emit_terminal_bytes(&socket, "stdout", err_msg.as_bytes().to_vec());
            emit_error(&socket, format!("could not start SSH terminal: {error}"));
            return;
        }
    };

    let (reader, writer) = pty.into_split();
    let child_arc = Arc::new(Mutex::new(child));
    let session_id = next_session_id();
    let cancel = tokio_util::sync::CancellationToken::new();

    sessions.insert(
        key.clone(),
        TerminalSession::Pty {
            writer: Arc::new(Mutex::new(writer)),
            child: child_arc.clone(),
            session_id,
            cancel: cancel.clone(),
        },
    );

    let _ = socket.emit(
        "started",
        &TerminalStarted {
            kind: "remote-server",
            host: Some(&actual_host),
        },
    );

    spawn_pty_reader(socket.clone(), reader);

    let sessions_clone = sessions.clone();
    let socket_clone = socket.clone();
    let server_host = actual_host.clone();
    let shell_bin_log = shell_bin.to_string();
    tokio::spawn(async move {
        let _keep_alive_agent = agent_session;
        let _keep_alive_askpass = temp_askpass;

        // Wait for either the child to exit naturally or a cancel signal (shell switch / disconnect)
        let status = tokio::select! {
            s = async { child_arc.lock().await.wait().await } => {
                Some(s)
            }
            _ = cancel.cancelled() => {
                // Kill the child process cleanly on cancel
                let _ = child_arc.lock().await.kill().await;
                None
            }
        };
        
        let is_current = match sessions_clone.get(&key) {
            Some(entry) => match entry.value() {
                TerminalSession::Pty { session_id: sid, .. } => *sid == session_id,
                _ => false,
            },
            None => false,
        };

        if is_current {
            sessions_clone.remove(&key);
            let code = status.and_then(|s| s.ok()).and_then(|s| s.code());
            tracing::info!(shell = %shell_bin_log, exit_code = ?code, host = %server_host, "remote terminal session exited");
            if let Some(c) = code {
                if c != 0 {
                    let err_msg = format!("\r\n\x1b[31m[Error] SSH session to server '{server_host}' closed with exit code {c}.\x1b[0m\r\n");
                    emit_terminal_bytes(&socket_clone, "stdout", err_msg.as_bytes().to_vec());
                }
            }
            let _ = socket_clone.emit("exit", &TerminalExit { code });
        }
    });
}
