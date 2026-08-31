use crate::api::proto::{
    shell_message, ExecTaskOutput, ExecTaskOutputKind, ExecTaskRequest, ShellError, ShellExit,
    ShellInput, ShellMessage, ShellOpen, ShellOutput,
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
        if request.server_id <= 0 || (server_id != -1 && request.server_id != server_id) {
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
            .stdin(if request.stdin.is_empty() {
                std::process::Stdio::null()
            } else {
                std::process::Stdio::piped()
            })
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
        let stdin_data = request.stdin;
        tokio::spawn(async move {
            let stdout_task = tokio::spawn(stream_task_output(
                stdout,
                ExecTaskOutputKind::ExecTaskOutputStdout,
                tx.clone(),
            ));
            let stderr_task = tokio::spawn(stream_task_output(
                stderr,
                ExecTaskOutputKind::ExecTaskOutputStderr,
                tx.clone(),
            ));
            if let Some(mut stdin) = child.stdin.take() {
                tokio::spawn(async move {
                    let _ = stdin.write_all(&stdin_data).await;
                    let _ = stdin.shutdown().await;
                });
            }
            let status = tokio::select! {
                status = child.wait() => status,
                _ = tokio::time::sleep(std::time::Duration::from_secs(timeout.max(1) as u64)), if timeout > 0 => {
                    let _ = child.kill().await;
                    let status = child.wait().await;
                    let _ = tx.send(Ok(ExecTaskOutput {
                        kind: ExecTaskOutputKind::ExecTaskOutputError as i32,
                        error: format!("task timed out after {timeout} seconds"),
                        ..Default::default()
                    })).await;
                    status
                },
                _ = tx.closed() => {
                    let _ = child.kill().await;
                    child.wait().await
                }
            };
            let _ = stdout_task.await;
            let _ = stderr_task.await;
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
        let open = match first.payload {
            Some(shell_message::Payload::Open(open)) => open,
            _ => return Err(Status::invalid_argument("first shell message must be open")),
        };
        if open.server_id <= 0
            || server_id != expected_server_id
            || (expected_server_id != -1 && open.server_id != expected_server_id)
        {
            return Err(Status::not_found(
                "shell server_id does not match this agent",
            ));
        }

        if !open.container.is_empty() {
            return Self::open_container(input, open).await;
        }

        let shell = if open.shell.is_empty() {
            "sh"
        } else {
            open.shell.as_str()
        };
        let mut command = Command::new(shell);
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
                        Ok(Some(ShellMessage { payload: Some(shell_message::Payload::Input(ShellInput { data })) })) => {
                            if stdin.write_all(&data).await.is_err() { break; }
                            let _ = stdin.flush().await;
                        }
                        Ok(Some(ShellMessage { payload: Some(shell_message::Payload::Close(_)) })) => {
                            let _ = stdin.shutdown().await;
                            break;
                        }
                        Ok(Some(_)) => {}
                        _ => break,
                    },
                    result = stdout.read(&mut out) => match result {
                        Ok(0) => {},
                        Ok(size) => { if tx.send(Ok(stdout_message(&out[..size]))).await.is_err() { break; } },
                        Err(error) => { let _ = tx.send(Ok(error_message(error.to_string()))).await; break; }
                    },
                    result = stderr.read(&mut err) => match result {
                        Ok(0) => {},
                        Ok(size) => { if tx.send(Ok(stderr_message(&err[..size]))).await.is_err() { break; } },
                        Err(error) => { let _ = tx.send(Ok(error_message(error.to_string()))).await; break; }
                    },
                    status = child.wait() => {
                        let code = status.ok().and_then(|status| status.code()).unwrap_or(-1);
                        let _ = tx.send(Ok(exit_message(code))).await;
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

    async fn open_container(
        mut input: tonic::Streaming<ShellMessage>,
        open: ShellOpen,
    ) -> Result<Response<ShellStream>, Status> {
        let docker = os::docker::DockerCli::new_local();
        let mut target = open.container.clone();
        if let Ok(containers) = docker.containers().ps().all().api().list().await {
            let search = target.to_lowercase();
            let clean_search = search.trim_end_matches("_db").trim_end_matches("-db");
            if let Some(found) = containers.iter().find(|container| {
                let name = container.names.to_lowercase();
                name.contains(&search)
                    || name.trim_start_matches('/').starts_with(&search)
                    || name.contains(&format!("{}_", search))
                    || name.contains(&format!("{}.", search))
                    || (!clean_search.is_empty()
                        && (name.contains(clean_search)
                            || name.trim_start_matches('/').starts_with(clean_search)))
            }) {
                target = if found.id.is_empty() {
                    found.names.trim_start_matches('/').to_owned()
                } else {
                    found.id.clone()
                };
            }
        }
        let shell = if open.shell.is_empty() {
            "sh".to_owned()
        } else {
            open.shell.clone()
        };
        let stream = docker
            .containers()
            .exec(&target)
            .interactive()
            .tty(true)
            .api()
            .env("TERM", "xterm-256color")
            .shell([shell])
            .await
            .map_err(|error| Status::internal(format!("could not start Docker shell: {error}")))?;
        let cols = u16::try_from(open.cols.max(1)).unwrap_or(u16::MAX);
        let rows = u16::try_from(open.rows.max(1)).unwrap_or(u16::MAX);
        let _ = stream.resize(cols, rows).await;
        let mut reader = stream.reader;
        let mut writer = stream.writer;
        let exec_id = stream.exec_id;
        let socket_path = stream.socket_path;
        let (tx, rx) = tokio::sync::mpsc::channel(32);

        tokio::spawn(async move {
            let mut output = [0u8; 4096];
            loop {
                tokio::select! {
                    message = input.message() => match message {
                        Ok(Some(ShellMessage { payload: Some(shell_message::Payload::Input(ShellInput { data })) })) => {
                            if writer.write_all(&data).await.is_err() { break; }
                            let _ = writer.flush().await;
                        }
                        Ok(Some(ShellMessage { payload: Some(shell_message::Payload::Resize(resize)) })) => {
                            let cols = u16::try_from(resize.cols.max(1)).unwrap_or(u16::MAX);
                            let rows = u16::try_from(resize.rows.max(1)).unwrap_or(u16::MAX);
                            let _ = os::docker::handles::containers::socat::resize_container_exec(
                                &socket_path, &exec_id, cols, rows,
                            ).await;
                        }
                        Ok(Some(ShellMessage { payload: Some(shell_message::Payload::Close(_)) })) => {
                            let _ = writer.shutdown().await;
                            break;
                        }
                        Ok(Some(_)) => {}
                        _ => break,
                    },
                    result = reader.read(&mut output) => match result {
                        Ok(0) => {
                            let _ = tx.send(Ok(exit_message(0))).await;
                            break;
                        }
                        Ok(size) => {
                            if tx.send(Ok(stdout_message(&output[..size]))).await.is_err() { break; }
                        }
                        Err(error) => {
                            let _ = tx.send(Ok(error_message(error.to_string()))).await;
                            break;
                        }
                    }
                }
            }
        });

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(rx),
        )))
    }
}

fn stdout_message(data: &[u8]) -> ShellMessage {
    ShellMessage {
        payload: Some(shell_message::Payload::Stdout(ShellOutput {
            data: data.to_vec(),
        })),
    }
}

fn stderr_message(data: &[u8]) -> ShellMessage {
    ShellMessage {
        payload: Some(shell_message::Payload::Stderr(ShellOutput {
            data: data.to_vec(),
        })),
    }
}

fn exit_message(code: i32) -> ShellMessage {
    ShellMessage {
        payload: Some(shell_message::Payload::Exit(ShellExit { exit_code: code })),
    }
}

fn error_message(message: String) -> ShellMessage {
    ShellMessage {
        payload: Some(shell_message::Payload::Error(ShellError { message })),
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
