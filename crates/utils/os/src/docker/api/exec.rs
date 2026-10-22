use super::DockerApi;
use crate::docker::{DockerError, DockerOutput, DockerResult};

pub async fn execute(api: &DockerApi, args: &[String]) -> DockerResult<DockerOutput> {
    let (id, tty, detach) = create(api, args).await?;
    let start = serde_json::json!({"Detach": detach, "Tty": tty});
    let payload = api
        .request(
            "POST",
            &format!("/exec/{id}/start"),
            Some(serde_json::to_vec(&start).unwrap().into()),
        )
        .await?;
    Ok(DockerOutput {
        status: crate::exec::ExecExitStatus::Api(200),
        stdout: String::from_utf8_lossy(&payload).into_owned(),
        stderr: String::new(),
    })
}

pub async fn execute_stream(
    api: &DockerApi,
    args: &[String],
    sender: tokio::sync::mpsc::Sender<crate::docker::DockerStreamEvent>,
) -> DockerResult<crate::docker::DockerExitStatus> {
    let (id, tty, detach) = create(api, args).await?;
    let start = serde_json::json!({"Detach": detach, "Tty": tty});
    let status = api
        .request_stream(
            "POST",
            &format!("/exec/{id}/start"),
            Some(serde_json::to_vec(&start).unwrap().into()),
            sender,
        )
        .await?;
    Ok(crate::exec::ExecExitStatus::Api(status))
}

async fn create(api: &DockerApi, args: &[String]) -> DockerResult<(String, bool, bool)> {
    let container = positional(args, 2)?;
    let mut command = Vec::new();
    let mut env = Vec::new();
    let mut user = None;
    let mut workdir = None;
    let mut tty = false;
    let mut detach = false;
    let mut seen_container = false;
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "-u" | "--user" => {
                user = args.get(i + 1).cloned();
                i += 2;
            }
            "-w" | "--workdir" => {
                workdir = args.get(i + 1).cloned();
                i += 2;
            }
            "--env" | "-e" => {
                env.push(required(args.get(i + 1), "exec env")?.clone());
                i += 2;
            }
            "-t" | "--tty" => {
                tty = true;
                i += 1;
            }
            "-i" | "--interactive" => i += 1,
            "-d" | "--detach" => {
                detach = true;
                i += 1;
            }
            value if !value.starts_with('-') && !seen_container => {
                seen_container = true;
                i += 1;
            }
            value if !value.starts_with('-') => {
                command.extend(args[i..].iter().cloned());
                break;
            }
            _ => i += 1,
        }
    }
    if command.is_empty() {
        return Err(api_error("container exec requires a command"));
    }
    let mut create = serde_json::json!({"AttachStdin": true, "AttachStdout": true, "AttachStderr": true, "Tty": tty, "Cmd": command, "Env": env});
    if let Some(value) = user {
        create["User"] = serde_json::json!(value);
    }
    if let Some(value) = workdir {
        create["WorkingDir"] = serde_json::json!(value);
    }
    let payload = api
        .request(
            "POST",
            &format!("/containers/{container}/exec"),
            Some(serde_json::to_vec(&create).unwrap().into()),
        )
        .await?;
    let id = serde_json::from_slice::<serde_json::Value>(&payload)
        .ok()
        .and_then(|value| {
            value
                .get("Id")
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned)
        })
        .ok_or_else(|| api_error("Docker exec create returned no Id"))?;
    Ok((id, tty, detach))
}

fn positional(args: &[String], start: usize) -> DockerResult<String> {
    let mut i = start;
    while i < args.len() {
        match args[i].as_str() {
            "-u" | "--user" | "-w" | "--workdir" | "--env" | "-e" => i += 2,
            value if value.starts_with('-') => i += 1,
            _ => return Ok(args[i].clone()),
        }
    }
    Err(api_error("container exec requires a container id"))
}

fn required<'a>(value: Option<&'a String>, name: &str) -> DockerResult<&'a String> {
    value.ok_or_else(|| api_error(format!("missing {name}")))
}

fn api_error(message: impl Into<String>) -> DockerError {
    DockerError::Docker(message.into())
}
