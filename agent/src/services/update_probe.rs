use serde_json::Value;
use tokio_stream::StreamExt;

use crate::api::proto::AgentInfoResponse;
use crate::docker::api::DockerApi;

const INSPECT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);
const PULL_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(180);

/// Fills the self-update probe fields of `AgentInfoResponse` through the same
/// Docker socket the monitoring reads use, so the panel's periodic update
/// check never spawns the docker CLI inside the agent's cgroup (that 28 MiB
/// process was the five-minute memory spike). Nothing is cached: every field
/// is read live at request time and failures simply leave the fields empty,
/// which makes the panel fall back to its out-of-band repair path.
pub async fn fill(docker: &DockerApi, info: &mut AgentInfoResponse, update_image: &str) {
    let Some(container_id) = self_container_id() else {
        return;
    };
    let inspected = tokio::time::timeout(
        INSPECT_TIMEOUT,
        docker.get_json::<Value>(&format!("/containers/{container_id}/json")),
    )
    .await
    .ok()
    .and_then(|result| result.ok());
    if let Some(container) = inspected {
        info.self_image_id = container
            .get("Image")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        info.self_running = container
            .get("State")
            .and_then(|state| state.get("Running"))
            .and_then(Value::as_bool)
            .unwrap_or(false);
        info.self_server_id = container
            .get("Config")
            .and_then(|config| config.get("Env"))
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .find_map(|entry| entry.strip_prefix("SERVER_ID=")?.parse::<i64>().ok())
            .unwrap_or(0);
    }
    if !update_image.is_empty() {
        info.latest_image_id = pull_and_inspect(docker, update_image).await;
    }
}

async fn pull_and_inspect(docker: &DockerApi, image: &str) -> String {
    let (name, tag) = split_image(image);
    let path = format!("/images/create?fromImage={name}&tag={tag}");
    let pull = async {
        let mut stream = match docker.post_stream(&path).await {
            Ok(stream) => stream,
            Err(error) => {
                tracing::debug!(%error, "update probe pull could not start");
                return false;
            }
        };
        // The progress body is consumed and discarded; completion of the
        // stream is the pull's success signal.
        while let Some(chunk) = stream.next().await {
            if chunk.is_err() {
                tracing::debug!("update probe pull stream failed");
                return false;
            }
        }
        true
    };
    match tokio::time::timeout(PULL_TIMEOUT, pull).await {
        Ok(true) => {}
        Ok(false) => return String::new(),
        Err(_) => {
            tracing::debug!("update probe pull timed out");
            return String::new();
        }
    }
    tokio::time::timeout(
        INSPECT_TIMEOUT,
        docker.get_json::<Value>(&format!("/images/{name}:{tag}/json")),
    )
    .await
    .ok()
    .and_then(|result| result.ok())
    .map(|image| {
        image
            .get("Id")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string()
    })
    .unwrap_or_default()
}

/// The agent's own container id, read live per request — no state. With a
/// host-shared cgroup path (`0::/system.slice/docker-<64hex>.scope`) the full
/// id comes from /proc/1/cgroup; Docker's default private cgroup namespace
/// hides it (`0::/`), so fall back to the kernel hostname, which Docker sets
/// to the container's 12-char short id and the Engine API resolves.
fn self_container_id() -> Option<String> {
    if let Ok(raw) = std::fs::read_to_string("/proc/1/cgroup")
        && let Some(id) = extract_container_id(&raw)
    {
        return Some(id);
    }
    let raw = std::fs::read_to_string("/proc/sys/kernel/hostname").ok()?;
    extract_short_container_id(&raw)
}

fn extract_container_id(raw: &str) -> Option<String> {
    raw.lines()
        .flat_map(|line| line.split('/'))
        .map(|part| {
            part.trim_start_matches("docker-")
                .trim_end_matches(".scope")
        })
        .find(|part| part.len() == 64 && part.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .map(str::to_ascii_lowercase)
}

fn extract_short_container_id(raw: &str) -> Option<String> {
    let host = raw.trim();
    (host.len() == 12 && host.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .then(|| host.to_ascii_lowercase())
}

/// Splits `repo:tag` into Docker's pull query form. A `:` only starts a tag
/// when it appears after the last `/`, so registry `host:port` prefixes stay
/// part of the name; an untagged image pulls `latest` like the CLI does.
fn split_image(image: &str) -> (&str, &str) {
    match image.rfind(':') {
        Some(index) if !image[index + 1..].contains('/') => (&image[..index], &image[index + 1..]),
        _ => (image, "latest"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_named_and_unnamed_images() {
        assert_eq!(
            split_image("dubeyanand/openoxide-agent:latest"),
            ("dubeyanand/openoxide-agent", "latest")
        );
        assert_eq!(
            split_image("dubeyanand/openoxide-agent"),
            ("dubeyanand/openoxide-agent", "latest")
        );
        assert_eq!(
            split_image("registry.local:5000/agent:v1"),
            ("registry.local:5000/agent", "v1")
        );
        assert_eq!(
            split_image("registry.local:5000/agent"),
            ("registry.local:5000/agent", "latest")
        );
    }

    #[test]
    fn extracts_ids_from_cgroup_form() {
        let raw = "0::/system.slice/docker-9c40c7061e00a1b2c3d4e5f60718293a4b5c6d7e8f90a1b2c3d4e5f60718293a.scope\n";
        assert_eq!(
            extract_container_id(raw).as_deref(),
            Some("9c40c7061e00a1b2c3d4e5f60718293a4b5c6d7e8f90a1b2c3d4e5f60718293a")
        );
        assert_eq!(extract_container_id("0::/init.scope"), None);
    }

    #[test]
    fn hostname_fallback_accepts_only_short_ids() {
        assert_eq!(
            extract_short_container_id("e04ff3832e61\n").as_deref(),
            Some("e04ff3832e61")
        );
        assert_eq!(extract_short_container_id("my-server"), None);
        assert_eq!(extract_short_container_id("e04ff3832e6"), None);
    }
}
