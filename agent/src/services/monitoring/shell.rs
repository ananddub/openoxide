use crate::api::proto::{
    ExecTaskOutput, ExecTaskOutputKind, ExecTaskRequest, ShellAction, ShellMessage,
};
use crate::docker::api::DockerApi;
use std::pin::Pin;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;
use tokio_stream::Stream;
use tonic::{Response, Status};

pub type ShellStream = Pin<Box<dyn Stream<Item = Result<ShellMessage, Status>> + Send + 'static>>;

pub struct ShellService;

impl ShellService {
    pub async fn execute(
        server_id: i64,
        request: ExecTaskRequest,
    ) -> Result<impl Stream<Item = Result<ExecTaskOutput, Status>> + Send + 'static, Status> {
        if request.server_id != server_id {
            return Err(Status::not_found(
                "task server_id does not match this agent",
            ));
        }
        if request.command.trim().is_empty() {
            return Err(Status::invalid_argument("task command cannot be empty"));
        }

        let mut command = if request.container.is_empty() {
            let mut command = Command::new("sh");
            command.args(["-lc", &request.command]);
            command
        } else {
            let mut command = Command::new("docker");
            command.args(["exec", "-i"]);
            if !request.working_directory.is_empty() {
                command.args(["-w", &request.working_directory]);
            }
            command.args([&request.container, "sh", "-lc", &request.command]);
            command
        };
        if request.container.is_empty() && !request.working_directory.is_empty() {
            command.current_dir(&request.working_directory);
        }
        let mut child = command
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .stdin(std::process::Stdio::null())
            .spawn()
            .map_err(|error| Status::internal(format!("could not start task: {error}")))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| Status::internal("task stdout unavailable"))?;
        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| Status::internal("task stderr unavailable"))?;
        let (tx, rx) = tokio::sync::mpsc::channel(32);
        let timeout = request.timeout_seconds;
        tokio::spawn(async move {
            let read_stdout =
                stream_task_output(stdout, ExecTaskOutputKind::ExecTaskOutputStdout, tx.clone());
            let read_stderr =
                stream_task_output(stderr, ExecTaskOutputKind::ExecTaskOutputStderr, tx.clone());
            tokio::join!(read_stdout, read_stderr);
            let status = if timeout == 0 {
                child.wait().await
            } else {
                match tokio::time::timeout(
                    std::time::Duration::from_secs(timeout as u64),
                    child.wait(),
                )
                .await
                {
                    Ok(status) => status,
                    Err(_) => {
                        let _ = child.kill().await;
                        child.wait().await
                    }
                }
            };
            let code = status.ok().and_then(|status| status.code()).unwrap_or(-1);
            let _ = tx
                .send(Ok(ExecTaskOutput {
                    kind: ExecTaskOutputKind::ExecTaskOutputExit as i32,
                    exit_code: code,
                    ..Default::default()
                }))
                .await;
        });
        Ok(tokio_stream::wrappers::ReceiverStream::new(rx))
    }

    pub async fn open(
        server_id: i64,
        mut input: tonic::Streaming<ShellMessage>,
        expected_server_id: i64,
        _docker: DockerApi,
    ) -> Result<Response<ShellStream>, Status> {
        let first = input
            .message()
            .await?
            .ok_or_else(|| Status::invalid_argument("missing shell open message"))?;
        if first.server_id != server_id || server_id != expected_server_id {
            return Err(Status::not_found(
                "shell server_id does not match this agent",
            ));
        }
        if first.action != ShellAction::Open as i32 {
            return Err(Status::invalid_argument("first shell message must be open"));
        }

        let shell = if first.shell.is_empty() {
            "sh"
        } else {
            first.shell.as_str()
        };
        let mut command = if first.container.is_empty() {
            Command::new(shell)
        } else {
            let mut command = Command::new("docker");
            command.args(["exec", "-i", &first.container, shell]);
            command
        };
        let mut child = command
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|error| Status::internal(format!("could not start shell: {error}")))?;
        let mut stdin = child
            .stdin
            .take()
            .ok_or_else(|| Status::internal("shell stdin unavailable"))?;
        let mut stdout = child
            .stdout
            .take()
            .ok_or_else(|| Status::internal("shell stdout unavailable"))?;
        let mut stderr = child
            .stderr
            .take()
            .ok_or_else(|| Status::internal("shell stderr unavailable"))?;
        let (tx, rx) = tokio::sync::mpsc::channel(32);

        tokio::spawn(async move {
            let mut out = [0u8; 4096];
            let mut err = [0u8; 4096];
            loop {
                tokio::select! {
                    message = input.message() => match message {
                        Ok(Some(message)) if message.action == ShellAction::Input as i32 => {
                            if stdin.write_all(&message.data).await.is_err() { break; }
                            let _ = stdin.flush().await;
                        }
                        Ok(Some(message)) if message.action == ShellAction::Close as i32 => {
                            let _ = stdin.shutdown().await;
                            break;
                        }
                        Ok(Some(_)) => {}
                        _ => break,
                    },
                    result = stdout.read(&mut out) => match result {
                        Ok(0) => {},
                        Ok(size) => { if tx.send(Ok(ShellMessage { action: ShellAction::Stdout as i32, data: out[..size].to_vec(), ..Default::default() })).await.is_err() { break; } },
                        Err(error) => { let _ = tx.send(Ok(ShellMessage { action: ShellAction::Error as i32, error: error.to_string(), ..Default::default() })).await; break; }
                    },
                    result = stderr.read(&mut err) => match result {
                        Ok(0) => {},
                        Ok(size) => { if tx.send(Ok(ShellMessage { action: ShellAction::Stderr as i32, data: err[..size].to_vec(), ..Default::default() })).await.is_err() { break; } },
                        Err(error) => { let _ = tx.send(Ok(ShellMessage { action: ShellAction::Error as i32, error: error.to_string(), ..Default::default() })).await; break; }
                    },
                    status = child.wait() => {
                        let code = status.ok().and_then(|status| status.code()).unwrap_or(-1);
                        let _ = tx.send(Ok(ShellMessage { action: ShellAction::Exit as i32, exit_code: code, ..Default::default() })).await;
                        break;
                    }
                }
            }
            let _ = child.kill().await;
        });

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(rx),
        )))
    }
}

async fn stream_task_output<R>(
    mut reader: R,
    kind: ExecTaskOutputKind,
    tx: tokio::sync::mpsc::Sender<Result<ExecTaskOutput, Status>>,
) where
    R: tokio::io::AsyncRead + Unpin,
{
    let mut buffer = [0_u8; 4096];
    loop {
        match reader.read(&mut buffer).await {
            Ok(0) => break,
            Ok(size) => {
                if tx
                    .send(Ok(ExecTaskOutput {
                        kind: kind as i32,
                        data: buffer[..size].to_vec(),
                        ..Default::default()
                    }))
                    .await
                    .is_err()
                {
                    break;
                }
            }
            Err(error) => {
                let _ = tx
                    .send(Ok(ExecTaskOutput {
                        kind: ExecTaskOutputKind::ExecTaskOutputError as i32,
                        error: error.to_string(),
                        ..Default::default()
                    }))
                    .await;
                break;
            }
        }
    }
}
