use uuid::Uuid;

use crate::db::queries::FlashQueries;
use crate::flash_gen::resource_dependencies::ServiceDependencyCountsParams;

use super::crud::{CreateDatabaseDto, PatchDatabaseDto};
use super::types::DatabaseKind;

/// Dependency counters for a managed database service.
///
/// Replaces the deleted `crate::repository::ResourceDependencyCounts`
/// (same shape and `blocks_delete` semantics; `poem_openapi` is no longer a
/// dependency, so the old `Object` derive is dropped).
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ResourceDependencyCounts {
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

impl ResourceDependencyCounts {
    pub fn blocks_delete(&self) -> bool {
        self.active_deployments > 0 || self.enabled_backups > 0
    }
}

pub(super) fn generate_app_name(name: &str, prefix: &str) -> String {
    let slug = slug_value(name);
    let suffix = Uuid::new_v4().simple().to_string();
    format!("{}-{}-{}", prefix, slug, &suffix[..6])
}

pub(super) fn slug_value(name: &str) -> String {
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
    if slug.is_empty() {
        "database".into()
    } else {
        slug.into()
    }
}

pub(super) fn random_secret() -> String {
    Uuid::new_v4().simple().to_string()
}

pub(super) fn db_type_value(kind: DatabaseKind) -> String {
    kind.as_str().to_uppercase()
}

/// engine_config JSON layout (per `.flash/schema/service_database.sql`):
/// - POSTGRES: `postgres_config` (object), `replication_role`,
///   `primary_host`, `primary_port`, `replication_user`,
///   `replication_password`
/// - MYSQL/MARIADB: `database_root_password`
/// - MONGO: `database_root_password`, `replica_sets`
/// - LIBSQL: `sqld_node`, `sqld_primary_url`, `enable_namespaces`,
///   `external_grpc_port`, `external_admin_port`
/// - REDIS: `{}` (empty object)
pub(super) fn build_engine_config(
    kind: DatabaseKind,
    input: &CreateDatabaseDto,
    root_password: &str,
) -> String {
    use serde_json::{Value, json};

    let mut config = serde_json::Map::new();
    match kind {
        DatabaseKind::Postgres => {
            config.insert("postgres_config".into(), json!({}));
            config.insert("replication_role".into(), json!("STANDALONE"));
        }
        DatabaseKind::Mysql | DatabaseKind::Mariadb | DatabaseKind::Mongo => {
            config.insert(
                "database_root_password".into(),
                json!(root_password.to_string()),
            );
            if kind == DatabaseKind::Mongo
                && let Some(replica_sets) = input.replica_sets
            {
                config.insert("replica_sets".into(), json!(replica_sets));
            }
        }
        DatabaseKind::Libsql => {
            if let Some(value) = &input.sqld_node {
                config.insert("sqld_node".into(), json!(value));
            }
            if let Some(value) = &input.sqld_primary_url {
                config.insert("sqld_primary_url".into(), json!(value));
            }
            if let Some(value) = input.enable_namespaces {
                config.insert("enable_namespaces".into(), json!(value));
            }
            if let Some(value) = input.external_grpc_port {
                config.insert("external_grpc_port".into(), json!(value));
            }
            if let Some(value) = input.external_admin_port {
                config.insert("external_admin_port".into(), json!(value));
            }
        }
        DatabaseKind::Redis => {}
    }
    Value::Object(config).to_string()
}

async fn current_engine_config(
    queries: &FlashQueries,
    service_id: i64,
) -> sqlx::Result<serde_json::Map<String, serde_json::Value>> {
    let row = queries.database_get_by_id(service_id).await?;
    Ok(row
        .engine_config
        .as_deref()
        .and_then(|value| serde_json::from_str(value).ok())
        .unwrap_or_default())
}

/// Merges the engine-specific fields of `PatchDatabaseDto` into the stored
/// `engine_config` JSON, keeping stored values for fields not provided.
pub(super) async fn patch_engine_config(
    queries: &FlashQueries,
    kind: DatabaseKind,
    service_id: i64,
    input: &PatchDatabaseDto,
) -> sqlx::Result<()> {
    use serde_json::{Value, json};

    let touched = input.database_root_password.is_some()
        || input.replica_sets.is_some()
        || input.sqld_node.is_some()
        || input.sqld_primary_url.is_some()
        || input.enable_namespaces.is_some()
        || input.external_grpc_port.is_some()
        || input.external_admin_port.is_some();
    if !touched {
        return Ok(());
    }

    let mut config = current_engine_config(queries, service_id).await?;
    match kind {
        DatabaseKind::Mysql | DatabaseKind::Mariadb | DatabaseKind::Mongo => {
            if let Some(value) = &input.database_root_password {
                config.insert("database_root_password".into(), json!(value.to_string()));
            }
            if kind == DatabaseKind::Mongo
                && let Some(value) = input.replica_sets
            {
                config.insert("replica_sets".into(), json!(value));
            }
        }
        DatabaseKind::Libsql => {
            if let Some(value) = &input.sqld_node {
                config.insert("sqld_node".into(), json!(value));
            }
            if let Some(value) = &input.sqld_primary_url {
                config.insert("sqld_primary_url".into(), json!(value));
            }
            if let Some(value) = input.enable_namespaces {
                config.insert("enable_namespaces".into(), json!(value));
            }
            if let Some(value) = input.external_grpc_port {
                config.insert("external_grpc_port".into(), json!(value));
            }
            if let Some(value) = input.external_admin_port {
                config.insert("external_admin_port".into(), json!(value));
            }
        }
        _ => {}
    }
    let serialized = Value::Object(config).to_string();
    queries
        .update_database_engine_config(&serialized, service_id)
        .await
}

pub(crate) async fn dependency_counts(
    queries: &FlashQueries,
    service_id: i64,
) -> sqlx::Result<ResourceDependencyCounts> {
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
    Ok(ResourceDependencyCounts {
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

/// No generated query covers `services.name`/`services.description` updates.
pub(crate) async fn update_service_identity(
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
