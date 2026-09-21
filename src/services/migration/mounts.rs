//! S4 inventory: turn a service's `mounts` rows (+ its implicit app-data
//! volume and source directory) into containment-checked [`PlannedObject`]s.
//!
//! VOLUME mounts are resolved to host paths via `docker volume inspect
//! -f '{{.Mountpoint}}'` on the SOURCE; unresolvable volumes (rootless or a
//! custom `--data-root`) yield [`MountIssue`]s so the caller can fall back to
//! the tar-stream `VolumeBackupRunner` path instead of syncing nothing.
//! BIND mounts carry their `host_path` directly. Content-only file mounts and
//! non-host mount types are skipped (regenerated on the target at redeploy).

use std::collections::{HashMap, HashSet};

use crate::db::queries::FlashQueries;
use crate::utils::exec::CommandExecutor;
use crate::utils::paths::OpenOxidePaths;

use super::planner::{PlanError, PlannedObject, plan_object};
use super::scope::MigrationObjectKind;

/// A mount that exists in the DB but has no usable host path on the source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MountIssue {
    pub service_id: i64,
    pub volume_name: String,
    pub reason: String,
}

/// argv for resolving a named volume's host mountpoint (docker is invoked as
/// argv, never a shell string).
pub fn volume_inspect_args(volume_name: &str) -> Result<Vec<String>, PlanError> {
    if volume_name.trim().is_empty()
        || volume_name.chars().any(char::is_whitespace)
        || volume_name.starts_with('-')
    {
        return Err(PlanError::UnsafeIdentifier(format!(
            "invalid volume name {volume_name:?}"
        )));
    }
    Ok(vec![
        "volume".into(),
        "inspect".into(),
        "-f".into(),
        "{{.Mountpoint}}".into(),
        volume_name.to_owned(),
    ])
}

/// Docker's canonical storage path for a named volume, used as a fallback
/// only when `docker volume inspect` is unavailable; a standard data root
/// makes this exact.
pub fn default_volume_path(volume_name: &str) -> Result<String, PlanError> {
    if volume_name.trim().is_empty() || volume_name.contains('/') || volume_name.starts_with('-') {
        return Err(PlanError::UnsafeIdentifier(format!(
            "invalid volume name {volume_name:?}"
        )));
    }
    Ok(format!("/var/lib/docker/volumes/{volume_name}/_data"))
}

/// Fetch every mount row for the scope's services.
pub async fn fetch_mount_rows(
    queries: &FlashQueries,
    application_ids: &[i64],
    compose_ids: &[i64],
) -> Result<Vec<crate::flash_gen::models::Mounts>, sqlx::Error> {
    let mut rows = Vec::new();
    for id in application_ids {
        rows.extend(queries.mount_fetch_for_application(*id).await?);
    }
    for id in compose_ids {
        rows.extend(queries.mount_fetch_for_compose(*id).await?);
    }
    Ok(rows)
}

/// Resolve named volumes to host paths via `docker volume inspect` on the
/// source; failures are reported per volume (never fatal to the whole plan).
pub async fn resolve_volume_paths(
    source: &CommandExecutor,
    volume_names: &HashSet<String>,
) -> HashMap<String, String> {
    let mut resolved = HashMap::new();
    for name in volume_names {
        let Ok(args) = volume_inspect_args(name) else {
            continue;
        };
        if let Ok(output) = source.run("docker", &args).await {
            let mountpoint = output.stdout.trim();
            if !mountpoint.is_empty() {
                resolved.insert(name.clone(), mountpoint.to_owned());
            }
        }
    }
    resolved
}

/// Build the ordered object plan (source dir → volumes → binds) for a scope.
/// `app_names`/`compose_names` map service id → directory name on disk.
/// Identical `obj_key`s across services dedupe naturally (same staging dir).
pub async fn plan_service_objects(
    paths: &OpenOxidePaths,
    migration_id: &str,
    queries: &FlashQueries,
    source: &CommandExecutor,
    application_ids: &[i64],
    compose_ids: &[i64],
    app_names: &HashMap<i64, String>,
    compose_names: &HashMap<i64, String>,
) -> Result<(Vec<PlannedObject>, Vec<MountIssue>), PlanError> {
    let mut objects: Vec<PlannedObject> = Vec::new();
    let mut issues = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    let push = |objects: &mut Vec<PlannedObject>,
                    seen: &mut HashSet<String>,
                    kind: MigrationObjectKind,
                    reference: &str|
     -> Result<(), PlanError> {
        let planned = plan_object(paths, migration_id, kind, reference)?;
        if seen.insert(planned.obj_key.clone()) {
            objects.push(planned);
        }
        Ok(())
    };

    // 1. Source-code directories (application `code`, compose `source`).
    for id in application_ids {
        if let Some(name) = app_names.get(id) {
            push(&mut objects, &mut seen, MigrationObjectKind::Src, &paths.application_code(name))
                .map_err(|_| {
                    PlanError::InvalidReference(format!("application dir name {name:?}"))
                })?;
        }
    }
    for id in compose_ids {
        if let Some(name) = compose_names.get(id) {
            push(&mut objects, &mut seen, MigrationObjectKind::Src, &paths.compose_source(name))
                .map_err(|_| {
                    PlanError::InvalidReference(format!("compose dir name {name:?}"))
                })?;
        }
    }

    // 2. Mount inventory: implicit `<app>-data` volumes, VOLUME (resolved via
    //    docker inspect), BIND (host path used verbatim).
    let rows = fetch_mount_rows(queries, application_ids, compose_ids)
        .await
        .map_err(|error| PlanError::InvalidReference(error.to_string()))?;

    let mut volume_names: HashSet<String> = HashSet::new();
    for id in application_ids {
        if let Some(name) = app_names.get(id) {
            volume_names.insert(format!("{name}-data"));
        }
    }
    for row in &rows {
        if row.mount_type == "VOLUME" {
            if let Some(volume) = row.volume_name.as_deref().map(str::trim) {
                if !volume.is_empty() {
                    volume_names.insert(volume.to_owned());
                }
            }
        }
    }
    let resolved = resolve_volume_paths(source, &volume_names).await;

    for volume in &volume_names {
        // No inspect result ⇒ report an issue (rootless/custom data-root must
        // fall back to the tar-stream path) — never guess the docker prefix
        // blindly at byte-moving time.
        let host_path = resolved.get(volume).cloned();
        match host_path {
            Some(path) => {
                if let Err(error) =
                    push(&mut objects, &mut seen, MigrationObjectKind::Volume, &path)
                {
                    issues.push(MountIssue {
                        service_id: 0,
                        volume_name: volume.clone(),
                        reason: error.to_string(),
                    });
                }
            }
            None => issues.push(MountIssue {
                service_id: 0,
                volume_name: volume.clone(),
                reason: "docker volume inspect returned no mountpoint (rootless or custom \
                         data-root? use tar-stream fallback)"
                    .into(),
            }),
        }
    }

    for row in &rows {
        match row.mount_type.as_str() {
            "VOLUME" => {} // handled above via the resolved set
            "BIND" => {
                let Some(host) = row.host_path.as_deref().map(str::trim) else {
                    continue;
                };
                if host.is_empty() {
                    continue;
                }
                // Relative / escaping bind paths are refused by the planner;
                // record an issue instead of dropping them silently.
                if let Err(error) = push(&mut objects, &mut seen, MigrationObjectKind::Bind, host) {
                    issues.push(MountIssue {
                        service_id: row.service_id,
                        volume_name: host.to_owned(),
                        reason: error.to_string(),
                    });
                }
            }
            // TMPFS / IMAGE / content-only file mounts carry no host bytes.
            _ => {}
        }
    }

    Ok((objects, issues))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn volume_inspect_argv_is_argv_safe() {
        let args = volume_inspect_args("app-data").unwrap();
        assert_eq!(
            args,
            vec![
                "volume",
                "inspect",
                "-f",
                "{{.Mountpoint}}",
                "app-data"
            ]
        );
        assert!(volume_inspect_args("").is_err());
        assert!(volume_inspect_args("app data").is_err());
        assert!(volume_inspect_args("--format=x").is_err());
    }

    #[test]
    fn default_volume_path_rejects_traversal() {
        assert_eq!(
            default_volume_path("app-data").unwrap(),
            "/var/lib/docker/volumes/app-data/_data"
        );
        assert!(default_volume_path("../../etc").is_err());
        assert!(default_volume_path("a/b").is_err());
    }

    #[test]
    fn planning_same_volume_twice_dedupes_by_obj_key() {
        // Two services sharing a named volume must produce ONE staged object
        // (same obj_key ⇒ same staging prefix ⇒ one rsync, one checkpoint).
        let paths = OpenOxidePaths {
            base: "/etc/openoxide".into(),
        };
        let a = plan_object(&paths, "m1", MigrationObjectKind::Volume, "/v/app-data/_data").unwrap();
        let b = plan_object(&paths, "m1", MigrationObjectKind::Volume, "/v/app-data/_data").unwrap();
        assert_eq!(a.obj_key, b.obj_key);
        let mut seen = HashSet::new();
        assert!(seen.insert(a.obj_key.clone()));
        assert!(!seen.insert(b.obj_key.clone()));
    }
}
