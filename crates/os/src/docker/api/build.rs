use super::{DockerApi, encode_query_public};
use crate::docker::{DockerError, DockerExitStatus, DockerResult, DockerStreamEvent};
use tokio::sync::mpsc;

pub async fn execute_stream(
    api: &DockerApi,
    args: &[String],
    sender: mpsc::Sender<DockerStreamEvent>,
) -> DockerResult<DockerExitStatus> {
    let context = args
        .last()
        .ok_or_else(|| error("image build requires context"))?;
    let (path, archive) = request_parts(args, context).await?;
    let status = api
        .request_stream_with_type(
            "POST",
            &path,
            Some(archive.into()),
            "application/x-tar",
            sender,
        )
        .await?;
    Ok(crate::exec::ExecExitStatus::Api(status))
}

async fn request_parts(args: &[String], context: &str) -> DockerResult<(String, Vec<u8>)> {
    let mut query = Vec::new();
    let mut build_args = serde_json::Map::new();
    let mut labels = serde_json::Map::new();
    for pair in args.windows(2) {
        match pair[0].as_str() {
            "--tag" | "-t" => query.push(format!("t={}", encode_query_public(&pair[1]))),
            "--file" | "-f" => query.push(format!("dockerfile={}", encode_query_public(&pair[1]))),
            "--target" => query.push(format!("target={}", encode_query_public(&pair[1]))),
            "--platform" => query.push(format!("platform={}", encode_query_public(&pair[1]))),
            "--cache-from" => query.push(format!(
                "cachefrom={}",
                encode_query_public(&serde_json::json!([pair[1]]).to_string())
            )),
            "--build-arg" => {
                let (key, value) = pair[1].split_once('=').unwrap_or((&pair[1], ""));
                build_args.insert(key.into(), serde_json::json!(value));
            }
            "--label" => {
                let (key, value) = pair[1].split_once('=').unwrap_or((&pair[1], ""));
                labels.insert(key.into(), serde_json::json!(value));
            }
            _ => {}
        }
    }
    if !build_args.is_empty() {
        query.push(format!(
            "buildargs={}",
            encode_query_public(&serde_json::Value::Object(build_args).to_string())
        ));
    }
    if !labels.is_empty() {
        query.push(format!(
            "labels={}",
            encode_query_public(&serde_json::Value::Object(labels).to_string())
        ));
    }
    query.push(format!(
        "nocache={}",
        args.iter().any(|arg| arg == "--no-cache")
    ));
    query.push(format!("pull={}", args.iter().any(|arg| arg == "--pull")));
    query.push("forcerm=true".into());
    let archive = tokio::process::Command::new("tar")
        .args(["-C", context, "-cf", "-", "."])
        .output()
        .await?;
    if !archive.status.success() {
        return Err(error(format!(
            "failed to archive build context: {}",
            String::from_utf8_lossy(&archive.stderr)
        )));
    }
    Ok((format!("/build?{}", query.join("&")), archive.stdout))
}

fn error(message: impl Into<String>) -> DockerError {
    DockerError::Ssh(message.into())
}
