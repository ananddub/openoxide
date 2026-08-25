-- Extracted from src/db/repository/server_private_networks.rs. Do not edit repository SQL directly.

-- name: ServerPrivateNetworksGet :one
SELECT server_id, connection_mode, provider, private_host, tunnel_address, public_key, endpoint, listen_port, persistent_keepalive, status, last_handshake_at, config_version, dns_name, routes, health_status, health_error, last_health_check_at, consecutive_failures, operation, operation_lease_until, config_hash, rotation_state, created_at, updated_at FROM server_private_networks WHERE server_id = ?;

-- name: ServerPrivateNetworksListManaged :many
SELECT server_id, connection_mode, provider, private_host, tunnel_address, public_key, endpoint, listen_port, persistent_keepalive, status, last_handshake_at, config_version, dns_name, routes, health_status, health_error, last_health_check_at, consecutive_failures, operation, operation_lease_until, config_hash, rotation_state, created_at, updated_at FROM server_private_networks WHERE connection_mode = ? AND status != ?;

-- name: ServerPrivateNetworksListDnsEnabled :many
SELECT server_id, connection_mode, provider, private_host, tunnel_address, public_key, endpoint, listen_port, persistent_keepalive, status, last_handshake_at, config_version, dns_name, routes, health_status, health_error, last_health_check_at, consecutive_failures, operation, operation_lease_until, config_hash, rotation_state, created_at, updated_at FROM server_private_networks WHERE dns_name IS NOT NULL AND status = ? ORDER BY dns_name;

-- name: ServerPrivateNetworksUpsert :exec
INSERT INTO server_private_networks (server_id, connection_mode, provider, private_host, tunnel_address, public_key, endpoint, listen_port, persistent_keepalive, dns_name, routes, status) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) ON CONFLICT(server_id) DO UPDATE SET connection_mode = excluded.connection_mode, provider = excluded.provider, private_host = excluded.private_host, tunnel_address = excluded.tunnel_address, public_key = excluded.public_key, endpoint = excluded.endpoint, listen_port = excluded.listen_port, persistent_keepalive = excluded.persistent_keepalive, dns_name = excluded.dns_name, routes = excluded.routes, status = excluded.status, config_version = server_private_networks.config_version + 1, updated_at = strftime('%s', 'now');

-- name: ServerPrivateNetworksDisable :exec
DELETE FROM server_private_networks WHERE server_id = ?;

-- name: ServerPrivateNetworksSetRuntimeState :exec
UPDATE server_private_networks SET status = ?, public_key = COALESCE(?, public_key), last_handshake_at = ?, updated_at = strftime('%s', 'now') WHERE server_id = ?;

-- name: ServerPrivateNetworksAcquireOperation :exec
UPDATE server_private_networks SET operation = ?, operation_lease_until = ?, updated_at = strftime('%s', 'now') WHERE server_id = ? AND (operation IS NULL OR operation_lease_until < ?);

-- name: ServerPrivateNetworksReleaseOperation :exec
UPDATE server_private_networks SET operation = NULL, operation_lease_until = NULL WHERE server_id = ? AND operation = ?;

-- name: ServerPrivateNetworksForceReleaseOperation :exec
UPDATE server_private_networks SET operation = NULL, operation_lease_until = NULL WHERE server_id = ?;

-- name: ServerPrivateNetworksRenewOperation :exec
UPDATE server_private_networks
               SET operation_lease_until = ?, updated_at = strftime('%s', 'now')
               WHERE server_id = ? AND operation = ?;

-- name: ServerPrivateNetworksSetHealth :exec
UPDATE server_private_networks SET health_status = ?, health_error = ?, last_handshake_at = COALESCE(?, last_handshake_at), last_health_check_at = strftime('%s', 'now'), consecutive_failures = CASE WHEN ? = ? THEN 0 ELSE consecutive_failures + 1 END, updated_at = strftime('%s', 'now') WHERE server_id = ?;

-- name: ServerPrivateNetworksBeginRotation :exec
UPDATE server_private_networks SET rotation_state = ? WHERE server_id = ?;

-- name: ServerPrivateNetworksFinishRotation :exec
UPDATE server_private_networks SET public_key = ?, rotation_state = ?, config_version = config_version + 1, updated_at = strftime('%s', 'now') WHERE server_id = ?;

-- name: ServerPrivateNetworksFailRotation :exec
UPDATE server_private_networks SET rotation_state = ? WHERE server_id = ?;
