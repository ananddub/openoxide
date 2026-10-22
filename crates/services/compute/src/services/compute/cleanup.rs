//! Removes a deleted service's runtime resources by their exact deploy names.

use std::collections::HashMap;
use std::time::Duration;

use crate::utils::docker::DockerCli;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CleanupRuntime {
    Stack,
    Compose,
}

/// Tears down the service's stack/project, named volumes and isolated network.
/// `volume_names` are the VOLUME-mount names read before the row cascade-deletes.
pub async fn cleanup_service_resources(
    docker: &DockerCli,
    app_name: &str,
    runtime: CleanupRuntime,
    volume_names: &[String],
) {
    match runtime {
        CleanupRuntime::Stack => {
            let _ = timeout(docker.stacks().remove(app_name).run()).await;
        }
        CleanupRuntime::Compose => {
            let _ = timeout(
                docker
                    .compose()
                    .down()
                    .project(app_name)
                    .remove_orphans()
                    .volumes()
                    .run(),
            )
            .await;
        }
    }

    // Named volumes survive stack/compose teardown, so remove each by name.
    // Only volumes labelled with this service's namespace are touched: an
    // explicitly mounted pre-existing volume is user data and must survive a
    // delete, and a name collision must never remove another app's volume.
    for volume in volume_names {
        if volume.trim().is_empty() {
            continue;
        }
        let inspected = match timeout(docker.volumes().inspect(volume.as_str())).await {
            Some(Ok(volume)) => volume,
            // Missing or unreadable volumes are not ours to remove.
            _ => continue,
        };
        if !volume_owned_by(&inspected.labels, runtime, app_name) {
            tracing::warn!(
                volume = %volume,
                service = %app_name,
                "skipping volume cleanup: volume is not labelled with this service's namespace"
            );
            continue;
        }
        // Stack/compose teardown returns before the old tasks finish
        // releasing their mounts, so an immediate `volume rm` fails with
        // "volume is in use" until the task's stop grace period elapses.
        if let Err(error) = remove_volume(docker, volume).await {
            tracing::warn!(
                volume = %volume,
                service = %app_name,
                %error,
                "could not remove owned volume during cleanup"
            );
        }
    }

    // No-op when the service had no isolated network.
    let _ = timeout(docker.networks().rm(app_name).run()).await;
}

async fn remove_volume(docker: &DockerCli, volume: &str) -> Result<(), String> {
    const ATTEMPTS: u32 = 60;
    let mut last_error = String::new();
    for attempt in 1..=ATTEMPTS {
        match timeout(docker.volumes().rm(volume).run()).await {
            Some(Ok(_)) => return Ok(()),
            Some(Err(error)) => last_error = error.to_string(),
            None => last_error = "docker volume rm timed out".into(),
        }
        if attempt < ATTEMPTS {
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }
    Err(last_error)
}

pub fn volume_names_from_mounts(mounts: &[crate::flash_gen::models::Mounts]) -> Vec<String> {
    mounts
        .iter()
        .filter(|mount| mount.mount_type.eq_ignore_ascii_case("VOLUME"))
        .filter_map(|mount| {
            mount
                .volume_name
                .as_deref()
                .map(str::trim)
                .filter(|name| !name.is_empty())
                .map(str::to_owned)
        })
        .collect()
}

/// Volumes a database deployment actually uses.
///
/// Mirrors the database stack builders: every VOLUME mount contributes its
/// name (or the generated name when unnamed), and a database with no mounts at
/// all gets `<app_name>-data`. Bind-only mount sets intentionally get no
/// generated volume. The delete path must use this instead of
/// [`volume_names_from_mounts`], which misses the generated volume and leaked
/// it on every delete.
pub fn database_volume_names(
    app_name: &str,
    mounts: &[crate::flash_gen::models::Mounts],
) -> Vec<String> {
    if mounts.is_empty() {
        return vec![default_database_volume(app_name)];
    }
    let mut names: Vec<String> = mounts
        .iter()
        // The builders only treat the exact uppercase tag as a volume.
        .filter(|mount| mount.mount_type == "VOLUME")
        .map(|mount| {
            mount
                .volume_name
                .as_deref()
                .map(str::trim)
                .filter(|name| !name.is_empty())
                .map(str::to_owned)
                .unwrap_or_else(|| default_database_volume(app_name))
        })
        .collect();
    names.sort();
    names.dedup();
    names
}

fn default_database_volume(app_name: &str) -> String {
    format!("{app_name}-data")
}

fn volume_owned_by(
    labels: &HashMap<String, String>,
    runtime: CleanupRuntime,
    app_name: &str,
) -> bool {
    let key = match runtime {
        CleanupRuntime::Stack => "com.docker.stack.namespace",
        CleanupRuntime::Compose => "com.docker.compose.project",
    };
    labels.get(key).is_some_and(|value| value == app_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flash_gen::models::Mounts;

    fn mount(mount_type: &str, volume_name: Option<&str>) -> Mounts {
        Mounts {
            id: 0,
            mount_type: mount_type.into(),
            service_type: "DATABASE".into(),
            host_path: None,
            volume_name: volume_name.map(str::to_owned),
            file_path: None,
            content: None,
            mount_path: "/data".into(),
            service_id: 1,
            created_at: 0,
            updated_at: 0,
        }
    }

    #[test]
    fn default_volume_is_derived_when_no_mounts_exist() {
        assert_eq!(
            database_volume_names("postgres-demo", &[]),
            vec!["postgres-demo-data"]
        );
    }

    #[test]
    fn unnamed_volume_mount_uses_generated_name() {
        assert_eq!(
            database_volume_names("pg", &[mount("VOLUME", None)]),
            vec!["pg-data"]
        );
        assert_eq!(
            database_volume_names("pg", &[mount("VOLUME", Some("  "))]),
            vec!["pg-data"]
        );
    }

    #[test]
    fn named_volume_mount_wins_and_is_deduplicated() {
        assert_eq!(
            database_volume_names(
                "pg",
                &[
                    mount("VOLUME", Some("pgdata")),
                    mount("VOLUME", Some("pgdata")),
                ]
            ),
            vec!["pgdata"]
        );
    }

    #[test]
    fn bind_only_mounts_do_not_get_a_generated_volume() {
        assert!(database_volume_names("pg", &[mount("BIND", None)]).is_empty());
        assert!(database_volume_names("pg", &[mount("FILE", None)]).is_empty());
    }

    #[test]
    fn ownership_requires_the_runtime_namespace_label() {
        let owned = HashMap::from([("com.docker.stack.namespace".to_string(), "pg".to_string())]);
        let foreign = HashMap::from([(
            "com.docker.stack.namespace".to_string(),
            "other".to_string(),
        )]);
        assert!(volume_owned_by(&owned, CleanupRuntime::Stack, "pg"));
        assert!(!volume_owned_by(&owned, CleanupRuntime::Compose, "pg"));
        assert!(!volume_owned_by(&foreign, CleanupRuntime::Stack, "pg"));
        assert!(!volume_owned_by(
            &HashMap::new(),
            CleanupRuntime::Stack,
            "pg"
        ));
    }
}

async fn timeout<F, T>(future: F) -> Option<T>
where
    F: std::future::Future<Output = T>,
{
    tokio::time::timeout(Duration::from_secs(5), future)
        .await
        .ok()
}
