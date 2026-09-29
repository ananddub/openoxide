//! Removes a deleted service's runtime resources by their exact deploy names.

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
    for volume in volume_names {
        if volume.trim().is_empty() {
            continue;
        }
        let _ = timeout(docker.volumes().rm(volume).run()).await;
    }

    // No-op when the service had no isolated network.
    let _ = timeout(docker.networks().rm(app_name).run()).await;
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

async fn timeout<F, T>(future: F) -> Option<T>
where
    F: std::future::Future<Output = T>,
{
    tokio::time::timeout(Duration::from_secs(5), future).await.ok()
}
