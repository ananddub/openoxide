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
        crate::db::reactive::request_scope(async {
            self.update_server(server_id, "ACTIVE").await?;
            self.reconcile_server_resources(server_id, states).await
        })
        .await
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
        sqlx::query(
            "UPDATE services SET app_status='ERROR' WHERE server_id=? AND app_status IN ('RUNNING','DONE','HEALTHY','OK','SUCCESS')",
        )
        .bind(server_id)
        .execute(self.pool.as_ref())
        .await
        .map_err(|error| error.to_string())?;
        Ok(())
    }

    async fn reconcile_server_resources(
        &self,
        server_id: i64,
        states: &[ContainerStatePoint],
    ) -> Result<(), String> {
        let local = self.is_local_server(server_id).await?;
        let scope = if local {
            "(s.server_id=? OR s.server_id IS NULL)"
        } else {
            "s.server_id=?"
        };
        let query = format!(
            "SELECT s.id, s.app_name, s.service_type, s.app_status AS current_status, \
             CASE WHEN s.service_type='APPLICATION' THEN MAX(s.replicas, 1) ELSE 1 END AS desired_replicas \
             FROM services s WHERE {scope}"
        );
        let rows = sqlx::query(sqlx::AssertSqlSafe(query))
            .bind(server_id)
            .fetch_all(self.pool.as_ref())
            .await
            .map_err(|error| error.to_string())?;

        for row in rows {
            let id: i64 = row.get("id");
            let app_name: String = row.get("app_name");
            let service_type: String = row.get("service_type");
            let current_status: String = row.get("current_status");
            let desired_replicas: i64 = row.get("desired_replicas");
            if !matches!(
                current_status.as_str(),
                "RUNNING" | "DONE" | "ERROR" | "HEALTHY" | "OK" | "SUCCESS"
            ) {
                continue;
            }
            let kind = match service_type.as_str() {
                "APPLICATION" => ResourceKind::Application,
                "COMPOSE" => ResourceKind::Compose,
                _ => ResourceKind::Database,
            };
            let matching: Vec<&ContainerStatePoint> = states
                .iter()
                .filter(|state| matches_resource(state, kind, id, &app_name))
                .collect();
            let status = aggregate_status(&matching, kind, desired_replicas).unwrap_or("ERROR");
            self.update_resource_status(id, status).await?;
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
            .is_some_and(|ip| matches!(ip.as_str(), "127.0.0.1" | "localhost" | "openoxide-agent")))
    }

    async fn update_resource_status(&self, id: i64, status: &str) -> Result<(), String> {
        let status = if status == "RUNNING" {
            "RUNNING"
        } else {
            "ERROR"
        };
        sqlx::query("UPDATE services SET app_status=? WHERE id=? AND app_status!=?")
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
