use uuid::Uuid;

use crate::db::queries::FlashQueries;
use crate::flash_gen::resource_dependencies::ServiceDependencyCountsParams;

pub(super) fn generate_app_name(name: &str) -> String {
    let mut slug = String::new();
    let mut previous_dash = false;

    for ch in name.to_lowercase().chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch);
            previous_dash = false;
        } else if !previous_dash && !slug.is_empty() {
            slug.push('-');
            previous_dash = true;
        }
    }

    let slug = slug.trim_matches('-');
    let base = if slug.is_empty() { "compose" } else { slug };
    let suffix = Uuid::new_v4().simple().to_string();
    format!("{}-{}", base, &suffix[..6])
}

pub(super) async fn update_git_watch_paths(
    queries: &FlashQueries,
    service_id: i64,
    watch_paths: Option<&str>,
) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE service_git SET watch_paths = COALESCE(?, watch_paths) WHERE service_id = ?",
    )
    .bind(watch_paths)
    .bind(service_id)
    .execute(&queries.pool)
    .await?;
    Ok(())
}

pub(super) async fn update_git_enable_submodules(
    queries: &FlashQueries,
    service_id: i64,
    enable_submodules: Option<i64>,
) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE service_git SET enable_submodules = COALESCE(?, enable_submodules) WHERE service_id = ?",
    )
    .bind(enable_submodules)
    .bind(service_id)
    .execute(&queries.pool)
    .await?;
    Ok(())
}

/// No generated query covers `services.name`/`services.description` updates.
pub(super) async fn update_service_identity(
    queries: &FlashQueries,
    id: i64,
    name: Option<&str>,
    description: Option<&str>,
) -> sqlx::Result<()> {
    sqlx::query(
        "UPDATE services SET name = COALESCE(?, name), description = COALESCE(?, description) WHERE id = ?",
    )
    .bind(name)
    .bind(description)
    .bind(id)
    .execute(&queries.pool)
    .await?;
    Ok(())
}

/// Dependency counters for a compose service.
///
/// Replaces the deleted `crate::repository::ResourceDependencyCounts`
/// (same shape and `blocks_delete` semantics). Kept local to the compose
/// tree so it stays decoupled from the other in-flight service migrations.
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ComposeDependencyCounts {
    pub active_deployments: i64,
    pub deployments: i64,
    pub domains: i64,
    pub mounts: i64,
    pub patches: i64,
    pub schedules: i64,
    pub backups: i64,
    pub enabled_backups: i64,
    pub preview_deployments: i64,
    pub middleware: i64,
    pub ports: i64,
    pub redirects: i64,
    pub security_rules: i64,
}

impl ComposeDependencyCounts {
    pub fn blocks_delete(&self) -> bool {
        self.active_deployments > 0 || self.enabled_backups > 0
    }
}

pub(super) async fn compose_dependency_counts(
    queries: &FlashQueries,
    service_id: i64,
) -> sqlx::Result<ComposeDependencyCounts> {
    let key = service_id.to_string();
    let params = ServiceDependencyCountsParams {
        service_id: key.clone(),
        service_id2: key.clone(),
        service_id3: key.clone(),
        service_id4: key.clone(),
        service_id5: key.clone(),
        service_id6: key.clone(),
        service_id7: key.clone(),
        service_id8: key.clone(),
        base_service_id: key.clone(),
        preview_service_id: key.clone(),
        service_id9: key.clone(),
        service_id10: key.clone(),
        service_id11: key.clone(),
        service_id12: key,
    };
    let row = queries.service_dependency_counts(&params).await?;
    Ok(ComposeDependencyCounts {
        active_deployments: row.active_deployments,
        deployments: row.deployments,
        domains: row.domains,
        mounts: row.mounts,
        patches: row.patches,
        schedules: row.schedules,
        backups: row.backups,
        enabled_backups: row.enabled_backups,
        preview_deployments: row.preview_deployments,
        middleware: row.middleware,
        ports: row.ports,
        redirects: row.redirects,
        security_rules: row.security_rules,
    })
}
