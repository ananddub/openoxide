use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;

use crate::db::models::server_private_networks::ServerPrivateNetwork;

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
            "SELECT server_id, connection_mode, provider, private_host, tunnel_address, public_key, endpoint, listen_port, persistent_keepalive, status, last_handshake_at, config_version, created_at, updated_at FROM server_private_networks WHERE server_id = ?",
        )
        .bind(server_id)
        .fetch_optional(self.pool.as_ref())
        .await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn upsert(
        &self,
        server_id: i64,
        connection_mode: &str,
        provider: Option<&str>,
        private_host: Option<&str>,
        tunnel_address: Option<&str>,
        public_key: Option<&str>,
        endpoint: Option<&str>,
        listen_port: Option<i64>,
        persistent_keepalive: Option<i64>,
        status: &str,
    ) -> sqlx::Result<ServerPrivateNetwork> {
        sqlx::query(
            "INSERT INTO server_private_networks (server_id, connection_mode, provider, private_host, tunnel_address, public_key, endpoint, listen_port, persistent_keepalive, status) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?) ON CONFLICT(server_id) DO UPDATE SET connection_mode = excluded.connection_mode, provider = excluded.provider, private_host = excluded.private_host, tunnel_address = excluded.tunnel_address, public_key = excluded.public_key, endpoint = excluded.endpoint, listen_port = excluded.listen_port, persistent_keepalive = excluded.persistent_keepalive, status = excluded.status, config_version = server_private_networks.config_version + 1, updated_at = strftime('%s', 'now')",
        )
        .bind(server_id)
        .bind(connection_mode)
        .bind(provider)
        .bind(private_host)
        .bind(tunnel_address)
        .bind(public_key)
        .bind(endpoint)
        .bind(listen_port)
        .bind(persistent_keepalive)
        .bind(status)
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
        status: &str,
        public_key: Option<&str>,
        last_handshake_at: Option<i64>,
    ) -> sqlx::Result<()> {
        sqlx::query("UPDATE server_private_networks SET status = ?, public_key = COALESCE(?, public_key), last_handshake_at = ?, updated_at = strftime('%s', 'now') WHERE server_id = ?")
            .bind(status)
            .bind(public_key)
            .bind(last_handshake_at)
            .bind(server_id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }
}
