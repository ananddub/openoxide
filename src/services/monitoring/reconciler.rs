use std::{collections::HashMap, sync::Arc};

use auto_di::singleton;
use sqlx::{Row, SqlitePool};

use super::monitoring_service::proto::ContainerStatePoint;

pub struct MonitoringReconciler {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl MonitoringReconciler {
    fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn apply_snapshot(
        &self,
        server_id: i64,
        states: &[ContainerStatePoint],
    ) -> Result<(), String> {
        self.update_server(server_id, "ACTIVE").await?;
        self.reconcile_server_resources(server_id, states).await?;
        // Agent lifecycle snapshots are background writes, so make the
        // affected live rooms explicit. This keeps the browser in sync for
        // direct Docker start/stop/restart events, including local hosts.
        auto_socket::notify_table_changes([
            "servers",
            "applications",
            "compose_projects",
            "postgres_dbs",
            "mysql_dbs",
            "mariadb_dbs",
            "mongo_dbs",
            "redis_dbs",
            "libsql_dbs",
            "deployments",
        ]);
        Ok(())
    }

    pub async fn mark_offline(&self, server_id: i64) -> Result<(), String> {
        self.update_server(server_id, "INACTIVE").await?;
        self.mark_server_resources_offline(server_id).await
    }

    async fn update_server(&self, id: i64, status: &str) -> Result<(), String> {
        sqlx::query("UPDATE servers SET server_status=? WHERE id=? AND server_status!=?")
            .bind(status)
            .bind(id)
            .bind(status)
            .execute(self.pool.as_ref())
            .await
            .map_err(|error| error.to_string())?;
        Ok(())
    }

    async fn mark_server_resources_offline(&self, server_id: i64) -> Result<(), String> {
        for (table, column) in resource_tables() {
            let query = format!(
                "UPDATE {table} SET {column}='ERROR' WHERE server_id=? AND {column} IN ('RUNNING','DONE','HEALTHY','OK','SUCCESS')"
            );
            sqlx::query(sqlx::AssertSqlSafe(query))
                .bind(server_id)
                .execute(self.pool.as_ref())
                .await
                .map_err(|error| error.to_string())?;
        }
        Ok(())
    }

    async fn reconcile_server_resources(
        &self,
        server_id: i64,
        states: &[ContainerStatePoint],
    ) -> Result<(), String> {
        self.reconcile_named_table("applications", server_id, states, ResourceKind::Application)
            .await?;
        self.reconcile_named_table("compose_projects", server_id, states, ResourceKind::Compose)
            .await?;
        for table in database_tables() {
            self.reconcile_named_table(table, server_id, states, ResourceKind::Database)
                .await?;
        }
        Ok(())
    }

    async fn reconcile_named_table(
        &self,
        table: &str,
        server_id: i64,
        states: &[ContainerStatePoint],
        kind: ResourceKind,
    ) -> Result<(), String> {
        let column = status_column(kind);
        let desired_replicas = if kind == ResourceKind::Application {
            "MAX(replicas, 1)"
        } else {
            "1"
        };
        let local = self.is_local_server(server_id).await?;
        let scope = if local {
            "(server_id=? OR server_id IS NULL)"
        } else {
            "server_id=?"
        };
        let query = format!(
            "SELECT id, app_name, {column} AS current_status, {desired_replicas} AS desired_replicas FROM {table} WHERE {scope}"
        );
        let rows = sqlx::query(sqlx::AssertSqlSafe(query))
            .bind(server_id)
            .fetch_all(self.pool.as_ref())
            .await
            .map_err(|error| error.to_string())?;

        for row in rows {
            let id: i64 = row.get("id");
            let app_name: String = row.get("app_name");
            let current_status: String = row.get("current_status");
            let desired_replicas: i64 = row.get("desired_replicas");
            if !matches!(
                current_status.as_str(),
                "RUNNING" | "DONE" | "ERROR" | "HEALTHY" | "OK" | "SUCCESS"
            ) {
                continue;
            }
            let matching: Vec<&ContainerStatePoint> = states
                .iter()
                .filter(|state| matches_resource(state, kind, id, &app_name))
                .collect();
            let status = aggregate_status(&matching, kind, desired_replicas).unwrap_or("ERROR");
            self.update_resource_status(table, column, id, status)
                .await?;
        }
        Ok(())
    }

    async fn is_local_server(&self, server_id: i64) -> Result<bool, String> {
        let row = sqlx::query("SELECT ip_address FROM servers WHERE id=?")
            .bind(server_id)
            .fetch_optional(self.pool.as_ref())
            .await
            .map_err(|error| error.to_string())?;
        Ok(row
            .and_then(|row| row.try_get::<String, _>("ip_address").ok())
            .is_some_and(|ip| {
                matches!(ip.as_str(), "127.0.0.1" | "localhost" | "openoxide-monitor")
            }))
    }

    async fn update_resource_status(
        &self,
        table: &str,
        column: &str,
        id: i64,
        status: &str,
    ) -> Result<(), String> {
        let status = if status == "RUNNING" {
            "RUNNING"
        } else {
            "ERROR"
        };
        let query = format!("UPDATE {table} SET {column}=? WHERE id=? AND {column}!=?");
        sqlx::query(sqlx::AssertSqlSafe(query))
            .bind(status)
            .bind(id)
            .bind(status)
            .execute(self.pool.as_ref())
            .await
            .map_err(|error| error.to_string())?;
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ResourceKind {
    Application,
    Compose,
    Database,
}

fn matches_resource(
    state: &ContainerStatePoint,
    kind: ResourceKind,
    id: i64,
    app_name: &str,
) -> bool {
    match kind {
        ResourceKind::Application => state.application_id == id || state.name.contains(app_name),
        ResourceKind::Compose => {
            state.compose_id == id
                || state.compose_project == app_name
                || state.name.contains(app_name)
        }
        ResourceKind::Database => state.name.contains(app_name),
    }
}

fn aggregate_status(
    states: &[&ContainerStatePoint],
    kind: ResourceKind,
    desired_replicas: i64,
) -> Option<&'static str> {
    if states.is_empty() {
        return None;
    }

    if kind == ResourceKind::Compose {
        let mut services: HashMap<&str, bool> = HashMap::new();
        for state in states {
            let service = if state.compose_service.is_empty() {
                state.name.as_str()
            } else {
                state.compose_service.as_str()
            };
            let running = state.state == "running";
            services
                .entry(service)
                .and_modify(|healthy| *healthy |= running)
                .or_insert(running);
        }
        return Some(if services.values().all(|running| *running) {
            "RUNNING"
        } else {
            "ERROR"
        });
    }

    let running = states
        .iter()
        .filter(|state| state.state == "running")
        .count();
    let expected = usize::try_from(desired_replicas.max(1)).unwrap_or(1);
    Some(if running >= expected {
        "RUNNING"
    } else {
        "ERROR"
    })
}

fn status_column(kind: ResourceKind) -> &'static str {
    match kind {
        ResourceKind::Application => "app_status",
        ResourceKind::Compose => "compose_status",
        ResourceKind::Database => "app_status",
    }
}

fn resource_tables() -> &'static [(&'static str, &'static str)] {
    &[
        ("applications", "app_status"),
        ("compose_projects", "compose_status"),
        ("postgres_dbs", "app_status"),
        ("mysql_dbs", "app_status"),
        ("mariadb_dbs", "app_status"),
        ("mongo_dbs", "app_status"),
        ("redis_dbs", "app_status"),
        ("libsql_dbs", "app_status"),
    ]
}

fn database_tables() -> &'static [&'static str] {
    &[
        "postgres_dbs",
        "mysql_dbs",
        "mariadb_dbs",
        "mongo_dbs",
        "redis_dbs",
        "libsql_dbs",
    ]
}
