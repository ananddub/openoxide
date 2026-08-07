use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;

use crate::db::models::server_private_networks::{
    PrivateNetworkConnectionMode, PrivateNetworkHealthStatus, PrivateNetworkOperation,
    PrivateNetworkProvider, PrivateNetworkRotationState, PrivateNetworkStatus,
    ServerPrivateNetwork,
};

pub struct ServerPrivateNetworkRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl ServerPrivateNetworkRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get(&self, server_id: i64) -> sqlx::Result<Option<ServerPrivateNetwork>> {
        sqlx::query_as::<_, ServerPrivateNetwork>(
            "SELECT server_id, connection_mode, provider, private_host, tunnel_address, public_key, endpoint, listen_port, persistent_keepalive, status, last_handshake_at, config_version, dns_name, routes, health_status, health_error, last_health_check_at, consecutive_failures, operation, operation_lease_until, config_hash, rotation_state, created_at, updated_at FROM server_private_networks WHERE server_id = ?",
        )
        .bind(server_id)
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn list_managed(&self) -> sqlx::Result<Vec<ServerPrivateNetwork>> {
        sqlx::query_as::<_, ServerPrivateNetwork>(
            "SELECT server_id, connection_mode, provider, private_host, tunnel_address, public_key, endpoint, listen_port, persistent_keepalive, status, last_handshake_at, config_version, dns_name, routes, health_status, health_error, last_health_check_at, consecutive_failures, operation, operation_lease_until, config_hash, rotation_state, created_at, updated_at FROM server_private_networks WHERE connection_mode = ? AND status != ?",
        )
        .bind(PrivateNetworkConnectionMode::ManagedWireguard.as_str())
        .bind(PrivateNetworkStatus::Disabled.as_str())
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn list_dns_enabled(&self) -> sqlx::Result<Vec<ServerPrivateNetwork>> {
        sqlx::query_as::<_, ServerPrivateNetwork>(
            "SELECT server_id, connection_mode, provider, private_host, tunnel_address, public_key, endpoint, listen_port, persistent_keepalive, status, last_handshake_at, config_version, dns_name, routes, health_status, health_error, last_health_check_at, consecutive_failures, operation, operation_lease_until, config_hash, rotation_state, created_at, updated_at FROM server_private_networks WHERE dns_name IS NOT NULL AND status = ? ORDER BY dns_name",
        )
        .bind(PrivateNetworkStatus::Active.as_str())
        .fetch_all(self.pool.as_ref())
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn upsert(
        &self,
        server_id: i64,
        connection_mode: PrivateNetworkConnectionMode,
        provider: Option<PrivateNetworkProvider>,
        private_host: Option<&str>,
        tunnel_address: Option<&str>,
        public_key: Option<&str>,
        endpoint: Option<&str>,
        listen_port: Option<i64>,
        persistent_keepalive: Option<i64>,
        dns_name: Option<&str>,
        routes: &str,
        status: PrivateNetworkStatus,
    ) -> sqlx::Result<ServerPrivateNetwork> {
        sqlx::query(
            "INSERT INTO server_private_networks (server_id, connection_mode, provider, private_host, tunnel_address, public_key, endpoint, listen_port, persistent_keepalive, dns_name, routes, status) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) ON CONFLICT(server_id) DO UPDATE SET connection_mode = excluded.connection_mode, provider = excluded.provider, private_host = excluded.private_host, tunnel_address = excluded.tunnel_address, public_key = excluded.public_key, endpoint = excluded.endpoint, listen_port = excluded.listen_port, persistent_keepalive = excluded.persistent_keepalive, dns_name = excluded.dns_name, routes = excluded.routes, status = excluded.status, config_version = server_private_networks.config_version + 1, updated_at = strftime('%s', 'now')",
        )
        .bind(server_id)
        .bind(connection_mode.as_str())
        .bind(provider.map(PrivateNetworkProvider::as_str))
        .bind(private_host)
        .bind(tunnel_address)
        .bind(public_key)
        .bind(endpoint)
        .bind(listen_port)
        .bind(persistent_keepalive)
        .bind(dns_name)
        .bind(routes)
        .bind(status.as_str())
        .execute(self.pool.as_ref())
        .await?;
        self.get(server_id).await?.ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn disable(&self, server_id: i64) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM server_private_networks WHERE server_id = ?")
            .bind(server_id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn set_runtime_state(
        &self,
        server_id: i64,
        status: PrivateNetworkStatus,
        public_key: Option<&str>,
        last_handshake_at: Option<i64>,
    ) -> sqlx::Result<()> {
        sqlx::query("UPDATE server_private_networks SET status = ?, public_key = COALESCE(?, public_key), last_handshake_at = ?, updated_at = strftime('%s', 'now') WHERE server_id = ?")
            .bind(status.as_str())
            .bind(public_key)
            .bind(last_handshake_at)
            .bind(server_id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn acquire_operation(
        &self,
        server_id: i64,
        operation: PrivateNetworkOperation,
        lease_seconds: i64,
    ) -> sqlx::Result<bool> {
        let now = chrono::Utc::now().timestamp();
        let result = sqlx::query("UPDATE server_private_networks SET operation = ?, operation_lease_until = ?, updated_at = strftime('%s', 'now') WHERE server_id = ? AND (operation IS NULL OR operation_lease_until < ?)")
            .bind(operation.as_str())
            .bind(now + lease_seconds)
            .bind(server_id)
            .bind(now)
            .execute(self.pool.as_ref())
            .await?;
        Ok(result.rows_affected() == 1)
    }

    pub async fn release_operation(
        &self,
        server_id: i64,
        operation: PrivateNetworkOperation,
    ) -> sqlx::Result<()> {
        sqlx::query("UPDATE server_private_networks SET operation = NULL, operation_lease_until = NULL WHERE server_id = ? AND operation = ?")
            .bind(server_id)
            .bind(operation.as_str())
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn renew_operation(
        &self,
        server_id: i64,
        operation: PrivateNetworkOperation,
        lease_seconds: i64,
    ) -> sqlx::Result<bool> {
        let lease_until = chrono::Utc::now().timestamp() + lease_seconds;
        let result = sqlx::query(
            r#"UPDATE server_private_networks
               SET operation_lease_until = ?, updated_at = strftime('%s', 'now')
               WHERE server_id = ? AND operation = ?"#,
        )
        .bind(lease_until)
        .bind(server_id)
        .bind(operation.as_str())
        .execute(self.pool.as_ref())
        .await?;
        Ok(result.rows_affected() == 1)
    }

    pub async fn set_health(
        &self,
        server_id: i64,
        status: PrivateNetworkHealthStatus,
        error: Option<&str>,
        handshake: Option<i64>,
    ) -> sqlx::Result<()> {
        sqlx::query("UPDATE server_private_networks SET health_status = ?, health_error = ?, last_handshake_at = COALESCE(?, last_handshake_at), last_health_check_at = strftime('%s', 'now'), consecutive_failures = CASE WHEN ? = ? THEN 0 ELSE consecutive_failures + 1 END, updated_at = strftime('%s', 'now') WHERE server_id = ?")
            .bind(status.as_str())
            .bind(error)
            .bind(handshake)
            .bind(status.as_str())
            .bind(PrivateNetworkHealthStatus::Healthy.as_str())
            .bind(server_id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn begin_rotation(&self, server_id: i64) -> sqlx::Result<()> {
        sqlx::query("UPDATE server_private_networks SET rotation_state = ? WHERE server_id = ?")
            .bind(PrivateNetworkRotationState::Rotating.as_str())
            .bind(server_id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn finish_rotation(&self, server_id: i64, public_key: &str) -> sqlx::Result<()> {
        sqlx::query("UPDATE server_private_networks SET public_key = ?, rotation_state = ?, config_version = config_version + 1, updated_at = strftime('%s', 'now') WHERE server_id = ?")
            .bind(public_key)
            .bind(PrivateNetworkRotationState::Idle.as_str())
            .bind(server_id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn fail_rotation(&self, server_id: i64) -> sqlx::Result<()> {
        sqlx::query("UPDATE server_private_networks SET rotation_state = ? WHERE server_id = ?")
            .bind(PrivateNetworkRotationState::Failed.as_str())
            .bind(server_id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
