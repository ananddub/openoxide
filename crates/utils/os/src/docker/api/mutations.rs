use super::DockerApi;
use crate::docker::{DockerError, DockerOutput, DockerResult};

pub fn handles(args: &[String]) -> bool {
    matches!(
        (
            args.first().map(String::as_str),
            args.get(1).map(String::as_str)
        ),
        (Some("service"), Some("update" | "scale" | "rollback"))
            | (Some("node"), Some("update" | "promote" | "demote"))
            | (Some("swarm"), Some("update"))
    )
}

pub async fn execute(api: &DockerApi, args: &[String]) -> DockerResult<DockerOutput> {
    match (
        args.first().map(String::as_str),
        args.get(1).map(String::as_str),
    ) {
        (Some("service"), Some("update")) => service_update(api, args).await,
        (Some("service"), Some("scale")) => service_scale(api, args).await,
        (Some("service"), Some("rollback")) => service_rollback(api, args).await,
        (Some("node"), Some("update" | "promote" | "demote")) => node_update(api, args).await,
        (Some("swarm"), Some("update")) => swarm_update(api, args).await,
        _ => Err(error("unsupported Docker mutation")),
    }
}

async fn swarm_update(api: &DockerApi, args: &[String]) -> DockerResult<DockerOutput> {
    let object: serde_json::Value = api.get_json("/swarm").await?;
    let version = object["Version"]["Index"]
        .as_u64()
        .ok_or_else(|| error("swarm has no Version.Index"))?;
    let mut spec = object
        .get("Spec")
        .cloned()
        .ok_or_else(|| error("swarm has no Spec"))?;
    for pair in args.windows(2) {
        match pair[0].as_str() {
            "--autolock" => spec["AutolockManagers"] = serde_json::json!(pair[1] == "true"),
            "--task-history-limit" => {
                spec["TaskHistory"] = serde_json::json!(pair[1].parse::<u64>().ok())
            }
            "--snapshot-interval" => {
                spec["Raft"]["SnapshotInterval"] = serde_json::json!(pair[1].parse::<u64>().ok())
            }
            "--dispatcher-heartbeat" => {
                spec["Dispatcher"]["HeartbeatPeriod"] =
                    serde_json::json!(parse_duration_ns(&pair[1]))
            }
            _ => {}
        }
    }
    update(api, "swarm", "update", version, spec, None).await
}

async fn service_update(api: &DockerApi, args: &[String]) -> DockerResult<DockerOutput> {
    let id = positional(
        args,
        2,
        &[
            "--image",
            "--replicas",
            "--env-add",
            "--env-rm",
            "--label-add",
            "--label-rm",
            "--limit-memory",
            "--update-parallelism",
            "--update-delay",
        ],
    )?;
    let (version, mut spec) = inspect_spec(api, "services", &id).await?;
    for pair in args.windows(2) {
        match pair[0].as_str() {
            "--image" => {
                spec["TaskTemplate"]["ContainerSpec"]["Image"] = serde_json::json!(pair[1])
            }
            "--replicas" => spec["Mode"]["Replicated"]["Replicas"] = number(&pair[1])?,
            "--env-add" => add_array(
                &mut spec["TaskTemplate"]["ContainerSpec"]["Env"],
                pair[1].clone(),
            ),
            "--env-rm" => {
                remove_prefixed(&mut spec["TaskTemplate"]["ContainerSpec"]["Env"], &pair[1])
            }
            "--label-add" => set_pair(&mut spec["Labels"], &pair[1])?,
            "--label-rm" => {
                if let Some(map) = spec["Labels"].as_object_mut() {
                    map.remove(&pair[1]);
                }
            }
            "--limit-memory" => {
                spec["TaskTemplate"]["Resources"]["Limits"]["MemoryBytes"] =
                    serde_json::json!(parse_bytes(&pair[1]))
            }
            "--update-parallelism" => spec["UpdateConfig"]["Parallelism"] = number(&pair[1])?,
            "--update-delay" => {
                spec["UpdateConfig"]["Delay"] = serde_json::json!(parse_duration_ns(&pair[1]))
            }
            _ => {}
        }
    }
    if args.iter().any(|arg| arg == "--force") {
        let current = spec["TaskTemplate"]["ForceUpdate"].as_u64().unwrap_or(0);
        spec["TaskTemplate"]["ForceUpdate"] = serde_json::json!(current + 1);
    }
    update(api, "services", &id, version, spec, None).await
}

async fn service_scale(api: &DockerApi, args: &[String]) -> DockerResult<DockerOutput> {
    let mut last = None;
    for item in args.iter().skip(2).filter(|item| !item.starts_with('-')) {
        let (id, replicas) = item
            .split_once('=')
            .ok_or_else(|| error("service scale expects SERVICE=REPLICAS"))?;
        let (version, mut spec) = inspect_spec(api, "services", id).await?;
        spec["Mode"]["Replicated"]["Replicas"] = number(replicas)?;
        last = Some(update(api, "services", id, version, spec, None).await?);
    }
    last.ok_or_else(|| error("service scale requires at least one service"))
}

async fn service_rollback(api: &DockerApi, args: &[String]) -> DockerResult<DockerOutput> {
    let id = args
        .get(2)
        .ok_or_else(|| error("service rollback requires service"))?;
    let (version, spec) = inspect_spec(api, "services", id).await?;
    update(
        api,
        "services",
        id,
        version,
        spec,
        Some("rollback=previous"),
    )
    .await
}

async fn node_update(api: &DockerApi, args: &[String]) -> DockerResult<DockerOutput> {
    let action = args.get(1).ok_or_else(|| error("node action required"))?;
    let id = positional(
        args,
        2,
        &["--availability", "--role", "--label-add", "--label-rm"],
    )?;
    let (version, mut spec) = inspect_spec(api, "nodes", &id).await?;
    if action == "promote" {
        spec["Role"] = serde_json::json!("manager");
    }
    if action == "demote" {
        spec["Role"] = serde_json::json!("worker");
    }
    for pair in args.windows(2) {
        match pair[0].as_str() {
            "--availability" => spec["Availability"] = serde_json::json!(pair[1]),
            "--role" => spec["Role"] = serde_json::json!(pair[1]),
            "--label-add" => set_pair(&mut spec["Labels"], &pair[1])?,
            "--label-rm" => {
                if let Some(map) = spec["Labels"].as_object_mut() {
                    map.remove(&pair[1]);
                }
            }
            _ => {}
        }
    }
    update(api, "nodes", &id, version, spec, None).await
}

async fn inspect_spec(
    api: &DockerApi,
    kind: &str,
    id: &str,
) -> DockerResult<(u64, serde_json::Value)> {
    let object: serde_json::Value = api.get_json(&format!("/{kind}/{id}")).await?;
    let version = object["Version"]["Index"]
        .as_u64()
        .ok_or_else(|| error("Docker object has no Version.Index"))?;
    let spec = object
        .get("Spec")
        .cloned()
        .ok_or_else(|| error("Docker object has no Spec"))?;
    Ok((version, spec))
}

async fn update(
    api: &DockerApi,
    kind: &str,
    id: &str,
    version: u64,
    spec: serde_json::Value,
    extra: Option<&str>,
) -> DockerResult<DockerOutput> {
    let suffix = extra.map(|value| format!("&{value}")).unwrap_or_default();
    let payload = api
        .request(
            "POST",
            &format!("/{kind}/{id}/update?version={version}{suffix}"),
            Some(serde_json::to_vec(&spec).unwrap().into()),
        )
        .await?;
    Ok(output(payload))
}

fn output(payload: bytes::Bytes) -> DockerOutput {
    DockerOutput {
        status: crate::exec::ExecExitStatus::Api(200),
        stdout: String::from_utf8_lossy(&payload).into_owned(),
        stderr: String::new(),
    }
}

fn positional(args: &[String], mut i: usize, valued: &[&str]) -> DockerResult<String> {
    while i < args.len() {
        if valued.contains(&args[i].as_str()) {
            i += 2;
        } else if args[i].starts_with('-') {
            i += 1;
        } else {
            return Ok(args[i].clone());
        }
    }
    Err(error("resource identifier required"))
}

fn add_array(target: &mut serde_json::Value, value: String) {
    if !target.is_array() {
        *target = serde_json::json!([]);
    }
    target
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!(value));
}
fn remove_prefixed(target: &mut serde_json::Value, key: &str) {
    if let Some(values) = target.as_array_mut() {
        values.retain(|value| {
            !value
                .as_str()
                .is_some_and(|value| value == key || value.starts_with(&format!("{key}=")))
        });
    }
}
fn set_pair(target: &mut serde_json::Value, pair: &str) -> DockerResult<()> {
    let (key, value) = pair
        .split_once('=')
        .ok_or_else(|| error("expected KEY=VALUE"))?;
    if !target.is_object() {
        *target = serde_json::json!({});
    }
    target[key] = serde_json::json!(value);
    Ok(())
}
fn number(value: &str) -> DockerResult<serde_json::Value> {
    Ok(serde_json::json!(
        value.parse::<u64>().map_err(|_| error("invalid number"))?
    ))
}
fn parse_bytes(value: &str) -> i64 {
    let value = value.to_ascii_lowercase();
    let (n, m) = value
        .strip_suffix('g')
        .map(|v| (v, 1_i64 << 30))
        .or_else(|| value.strip_suffix('m').map(|v| (v, 1_i64 << 20)))
        .or_else(|| value.strip_suffix('k').map(|v| (v, 1_i64 << 10)))
        .unwrap_or((&value, 1));
    n.parse::<f64>().map(|n| (n * m as f64) as i64).unwrap_or(0)
}
fn parse_duration_ns(value: &str) -> u64 {
    value
        .strip_suffix("ms")
        .and_then(|v| v.parse().ok())
        .map(|v: u64| v * 1_000_000)
        .or_else(|| {
            value
                .strip_suffix('s')
                .and_then(|v| v.parse().ok())
                .map(|v: u64| v * 1_000_000_000)
        })
        .unwrap_or(0)
}
fn error(message: impl Into<String>) -> DockerError {
    DockerError::Docker(message.into())
}
