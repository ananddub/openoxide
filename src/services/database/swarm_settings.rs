use crate::api::dto::database::DatabaseSwarmSettingsDto;
use crate::services::database::DatabaseKind;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct DatabaseSwarmSettingsService {
    pool: Arc<SqlitePool>,
}

#[auto_di::singleton]
impl DatabaseSwarmSettingsService {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    fn table(kind: DatabaseKind) -> &'static str {
        match kind {
            DatabaseKind::Postgres => "postgres_dbs",
            DatabaseKind::Mysql => "mysql_dbs",
            DatabaseKind::Mariadb => "mariadb_dbs",
            DatabaseKind::Mongo => "mongo_dbs",
            DatabaseKind::Redis => "redis_dbs",
            DatabaseKind::Libsql => "libsql_dbs",
        }
    }

    pub async fn get(&self, kind: DatabaseKind, id: i64) -> sqlx::Result<DatabaseSwarmSettingsDto> {
        let table = Self::table(kind);
        let sql = sqlx::AssertSqlSafe(format!(
            "SELECT health_check_swarm, restart_policy_swarm, placement_swarm, update_config_swarm, rollback_config_swarm, mode_swarm, labels_swarm, network_swarm, endpoint_spec_swarm, ulimits_swarm, stop_grace_period_swarm FROM {table} WHERE id = ?"
        ));
        sqlx::query_as::<_, DatabaseSwarmSettingsDto>(sql)
            .bind(id)
            .fetch_one(self.pool.as_ref())
            .await
    }

    pub async fn update(
        &self,
        kind: DatabaseKind,
        id: i64,
        value: DatabaseSwarmSettingsDto,
    ) -> sqlx::Result<()> {
        let table = Self::table(kind);
        let sql = sqlx::AssertSqlSafe(format!(
            "UPDATE {table} SET health_check_swarm = ?, restart_policy_swarm = ?, placement_swarm = ?, update_config_swarm = ?, rollback_config_swarm = ?, mode_swarm = ?, labels_swarm = ?, network_swarm = ?, endpoint_spec_swarm = ?, ulimits_swarm = ?, stop_grace_period_swarm = ? WHERE id = ?"
        ));
        sqlx::query(sql)
            .bind(value.health_check_swarm)
            .bind(value.restart_policy_swarm)
            .bind(value.placement_swarm)
            .bind(value.update_config_swarm)
            .bind(value.rollback_config_swarm)
            .bind(value.mode_swarm)
            .bind(value.labels_swarm)
            .bind(value.network_swarm)
            .bind(value.endpoint_spec_swarm)
            .bind(value.ulimits_swarm)
            .bind(value.stop_grace_period_swarm)
            .bind(id)
            .execute(self.pool.as_ref())
            .await
            .map(|_| ())
    }
}
