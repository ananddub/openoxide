use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Clone, Debug, Default, serde::Serialize, poem_openapi::Object)]
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

#[derive(Clone, Debug, Default, serde::Serialize, poem_openapi::Object)]
pub struct CertificateDependencyCounts {
    pub renewals: i64,
    pub running_renewals: i64,
}

#[derive(Clone, Debug, Default, serde::Serialize, poem_openapi::Object)]
pub struct NetworkDependencyCounts {
    pub applications: i64,
    pub compose_services: i64,
    pub databases: i64,
}

impl NetworkDependencyCounts {
    pub fn total(&self) -> i64 {
        self.applications + self.compose_services + self.databases
    }
}

pub struct ResourceDependencyRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl ResourceDependencyRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn application(&self, id: i64) -> sqlx::Result<ResourceDependencyCounts> {
        let row = sqlx::query!(
            r#"SELECT
                (SELECT COUNT(*) FROM deployments WHERE application_id = ? AND status IN ('QUEUED', 'RUNNING')) AS "active_deployments!: i64",
                (SELECT COUNT(*) FROM deployments WHERE application_id = ?) AS "deployments!: i64",
                (SELECT COUNT(*) FROM domains WHERE application_id = ?) AS "domains!: i64",
                (SELECT COUNT(*) FROM mounts WHERE application_id = ?) AS "mounts!: i64",
                (SELECT COUNT(*) FROM patches WHERE application_id = ?) AS "patches!: i64",
                (SELECT COUNT(*) FROM schedules WHERE application_id = ?) AS "schedules!: i64",
                (SELECT COUNT(*) FROM volume_backups WHERE application_id = ?) AS "backups!: i64",
                (SELECT COUNT(*) FROM volume_backups WHERE application_id = ? AND enabled = 1) AS "enabled_backups!: i64",
                (SELECT COUNT(*) FROM preview_deployments WHERE base_application_id = ? OR preview_application_id = ?) AS "preview_deployments!: i64",
                (SELECT COUNT(*) FROM application_middlewares WHERE application_id = ?) AS "middleware!: i64",
                (SELECT COUNT(*) FROM ports WHERE application_id = ?) AS "ports!: i64",
                (SELECT COUNT(*) FROM redirects WHERE application_id = ?) AS "redirects!: i64",
                (SELECT COUNT(*) FROM security WHERE application_id = ?) AS "security_rules!: i64""#,
            id, id, id, id, id, id, id, id, id, id, id, id, id, id
        )
        .fetch_one(self.pool.as_ref())
        .await?;
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

    pub async fn compose(&self, id: i64) -> sqlx::Result<ResourceDependencyCounts> {
        let row = sqlx::query!(
            r#"SELECT
                (SELECT COUNT(*) FROM deployments WHERE compose_id = ? AND status IN ('QUEUED', 'RUNNING')) AS "active_deployments!: i64",
                (SELECT COUNT(*) FROM deployments WHERE compose_id = ?) AS "deployments!: i64",
                (SELECT COUNT(*) FROM domains WHERE compose_id = ?) AS "domains!: i64",
                (SELECT COUNT(*) FROM mounts WHERE compose_id = ?) AS "mounts!: i64",
                (SELECT COUNT(*) FROM patches WHERE compose_id = ?) AS "patches!: i64",
                (SELECT COUNT(*) FROM schedules WHERE compose_id = ?) AS "schedules!: i64",
                ((SELECT COUNT(*) FROM backups WHERE compose_id = ?) + (SELECT COUNT(*) FROM volume_backups WHERE compose_id = ?)) AS "backups!: i64",
                ((SELECT COUNT(*) FROM backups WHERE compose_id = ? AND enabled = 1) + (SELECT COUNT(*) FROM volume_backups WHERE compose_id = ? AND enabled = 1)) AS "enabled_backups!: i64""#,
            id, id, id, id, id, id, id, id, id, id
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(ResourceDependencyCounts {
            active_deployments: row.active_deployments,
            deployments: row.deployments,
            domains: row.domains,
            mounts: row.mounts,
            patches: row.patches,
            schedules: row.schedules,
            backups: row.backups,
            enabled_backups: row.enabled_backups,
            ..Default::default()
        })
    }

    pub async fn certificate(&self, id: i64) -> sqlx::Result<CertificateDependencyCounts> {
        let row = sqlx::query!(
            r#"SELECT COUNT(*) AS "renewals!: i64",
                COUNT(*) FILTER (WHERE status = 'RUNNING') AS "running_renewals!: i64"
               FROM certificate_renewals WHERE certificate_id = ?"#,
            id
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(CertificateDependencyCounts {
            renewals: row.renewals,
            running_renewals: row.running_renewals,
        })
    }

    pub async fn database(&self, kind: &str, id: i64) -> sqlx::Result<ResourceDependencyCounts> {
        let row = sqlx::query!(
            r#"SELECT
                (SELECT COUNT(*) FROM deployments WHERE database_id = ? AND database_kind = ? AND status IN ('QUEUED', 'RUNNING')) AS "active_deployments!: i64",
                (SELECT COUNT(*) FROM deployments WHERE database_id = ? AND database_kind = ?) AS "deployments!: i64",
                (SELECT COUNT(*) FROM backups WHERE
                    (? = 'postgres' AND postgres_id = ?) OR (? = 'mysql' AND mysql_id = ?) OR
                    (? = 'mariadb' AND mariadb_id = ?) OR (? = 'mongo' AND mongo_id = ?) OR
                    (? = 'redis' AND redis_id = ?) OR (? = 'libsql' AND libsql_id = ?)) AS "backups!: i64",
                (SELECT COUNT(*) FROM backups WHERE enabled = 1 AND (
                    (? = 'postgres' AND postgres_id = ?) OR (? = 'mysql' AND mysql_id = ?) OR
                    (? = 'mariadb' AND mariadb_id = ?) OR (? = 'mongo' AND mongo_id = ?) OR
                    (? = 'redis' AND redis_id = ?) OR (? = 'libsql' AND libsql_id = ?))) AS "enabled_backups!: i64""#,
            id, kind, id, kind,
            kind, id, kind, id, kind, id, kind, id, kind, id, kind, id,
            kind, id, kind, id, kind, id, kind, id, kind, id, kind, id
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        let volume = sqlx::query!(
            r#"SELECT COUNT(*) AS "backups!: i64",
                COUNT(*) FILTER (WHERE enabled = 1) AS "enabled_backups!: i64"
               FROM volume_backups WHERE
                (? = 'postgres' AND postgres_id = ?) OR (? = 'mysql' AND mysql_id = ?) OR
                (? = 'mariadb' AND mariadb_id = ?) OR (? = 'mongo' AND mongo_id = ?) OR
                (? = 'redis' AND redis_id = ?) OR (? = 'libsql' AND libsql_id = ?)"#,
            kind,
            id,
            kind,
            id,
            kind,
            id,
            kind,
            id,
            kind,
            id,
            kind,
            id
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        Ok(ResourceDependencyCounts {
            active_deployments: row.active_deployments,
            deployments: row.deployments,
            backups: row.backups + volume.backups,
            enabled_backups: row.enabled_backups + volume.enabled_backups,
            ..Default::default()
        })
    }

    pub async fn database_network(&self, id: i64) -> sqlx::Result<NetworkDependencyCounts> {
        let key = id.to_string();
        let applications = sqlx::query_scalar!(
            r#"SELECT COUNT(*) AS "count!: i64" FROM applications, json_each(applications.network_ids) WHERE json_each.value = ?"#,
            key
        ).fetch_one(self.pool.as_ref()).await?;
        let compose_services = sqlx::query_scalar!(
            r#"SELECT COUNT(DISTINCT compose_projects.id) AS "count!: i64"
               FROM compose_projects, json_tree(compose_projects.service_networks)
               WHERE json_tree.value = ?"#,
            key
        )
        .fetch_one(self.pool.as_ref())
        .await?;
        let databases = sqlx::query_scalar!(
            r#"SELECT
                (SELECT COUNT(*) FROM postgres_dbs, json_each(postgres_dbs.network_ids) WHERE json_each.value = ?) +
                (SELECT COUNT(*) FROM mysql_dbs, json_each(mysql_dbs.network_ids) WHERE json_each.value = ?) +
                (SELECT COUNT(*) FROM mariadb_dbs, json_each(mariadb_dbs.network_ids) WHERE json_each.value = ?) +
                (SELECT COUNT(*) FROM mongo_dbs, json_each(mongo_dbs.network_ids) WHERE json_each.value = ?) +
                (SELECT COUNT(*) FROM redis_dbs, json_each(redis_dbs.network_ids) WHERE json_each.value = ?) +
                (SELECT COUNT(*) FROM libsql_dbs, json_each(libsql_dbs.network_ids) WHERE json_each.value = ?) AS "count!: i64""#,
            key, key, key, key, key, key
        ).fetch_one(self.pool.as_ref()).await?;
        Ok(NetworkDependencyCounts {
            applications,
            compose_services,
            databases,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{NetworkDependencyCounts, ResourceDependencyCounts};

    #[test]
    fn active_runtime_dependencies_block_delete() {
        assert!(
            ResourceDependencyCounts {
                active_deployments: 1,
                ..Default::default()
            }
            .blocks_delete()
        );
        assert!(
            ResourceDependencyCounts {
                enabled_backups: 1,
                ..Default::default()
            }
            .blocks_delete()
        );
        assert!(
            !ResourceDependencyCounts {
                deployments: 20,
                domains: 2,
                ..Default::default()
            }
            .blocks_delete()
        );
    }

    #[test]
    fn network_usage_total_includes_every_resource_class() {
        assert_eq!(
            NetworkDependencyCounts {
                applications: 2,
                compose_services: 3,
                databases: 4,
            }
            .total(),
            9
        );
    }
}
