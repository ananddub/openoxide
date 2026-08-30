-- Extracted from src/db/repository/server_private_networks.rs. Do not edit repository SQL directly.

-- name: ServerPrivateNetworksGet :one
SELECT server_id, connection_mode, provider, private_host, tunnel_address, public_key, endpoint, listen_port, persistent_keepalive, status, last_handshake_at, config_version, dns_name, routes, health_status, health_error, last_health_check_at, consecutive_failures, operation, operation_lease_until, config_hash, rotation_state, created_at, updated_at FROM server_private_networks WHERE server_id = ?;

-- name: ServerPrivateNetworksListManaged :many
SELECT server_id, connection_mode, provider, private_host, tunnel_address, public_key, endpoint, listen_port, persistent_keepalive, status, last_handshake_at, config_version, dns_name, routes, health_status, health_error, last_health_check_at, consecutive_failures, operation, operation_lease_until, config_hash, rotation_state, created_at, updated_at FROM server_private_networks WHERE connection_mode = ? AND status != ?;

-- name: ServerPrivateNetworksListDnsEnabled :many
SELECT server_id, connection_mode, provider, private_host, tunnel_address, public_key, endpoint, listen_port, persistent_keepalive, status, last_handshake_at, config_version, dns_name, routes, health_status, health_error, last_health_check_at, consecutive_failures, operation, operation_lease_until, config_hash, rotation_state, created_at, updated_at FROM server_private_networks WHERE dns_name IS NOT NULL AND dns_name != '' AND status = ? ORDER BY dns_name;

-- name: ServerPrivateNetworksInsert :exec
INSERT INTO server_private_networks (server_id, connection_mode, provider, private_host, tunnel_address, public_key, endpoint, listen_port, persistent_keepalive, dns_name, routes, status) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

-- name: ServerPrivateNetworksUpdateConfig :exec
UPDATE server_private_networks SET connection_mode = ?, provider = NULLIF(?, ''), private_host = NULLIF(?, ''), tunnel_address = NULLIF(?, ''), public_key = NULLIF(?, ''), endpoint = NULLIF(?, ''), listen_port = ?, persistent_keepalive = ?, dns_name = NULLIF(?, ''), routes = ?, status = ?, config_version = config_version + 1, updated_at = unixepoch() WHERE server_id = ?;

-- name: ServerPrivateNetworksDisable :exec
DELETE FROM server_private_networks WHERE server_id = ?;

-- name: ServerPrivateNetworksSetRuntimeStatus :exec
UPDATE server_private_networks SET status = ?, updated_at = unixepoch() WHERE server_id = ?;

-- name: ServerPrivateNetworksSetRuntimeStatusKey :exec
UPDATE server_private_networks SET status = ?, public_key = ?, updated_at = unixepoch() WHERE server_id = ?;

-- name: ServerPrivateNetworksSetRuntimeStatusHandshake :exec
UPDATE server_private_networks SET status = ?, last_handshake_at = ?, updated_at = unixepoch() WHERE server_id = ?;

-- name: ServerPrivateNetworksSetRuntimeFull :exec
UPDATE server_private_networks SET status = ?, public_key = ?, last_handshake_at = ?, updated_at = unixepoch() WHERE server_id = ?;

-- name: ServerPrivateNetworksAcquireOperation :one
UPDATE server_private_networks SET operation = ?, operation_lease_until = ?, updated_at = unixepoch() WHERE server_id = ? AND (operation IS NULL OR operation_lease_until < ?) RETURNING server_id;

-- name: ServerPrivateNetworksReleaseOperation :exec
UPDATE server_private_networks SET operation = NULL, operation_lease_until = NULL WHERE server_id = ? AND operation = ?;

-- name: ServerPrivateNetworksForceReleaseOperation :exec
UPDATE server_private_networks SET operation = NULL, operation_lease_until = NULL WHERE server_id = ?;

-- name: ServerPrivateNetworksRenewOperation :one
UPDATE server_private_networks
               SET operation_lease_until = ?, updated_at = unixepoch()
               WHERE server_id = ? AND operation = ?
               RETURNING server_id;

-- name: ServerPrivateNetworksSetHealth :exec
UPDATE server_private_networks SET health_status = ?, health_error = NULLIF(?, ''), last_health_check_at = unixepoch(), consecutive_failures = CASE WHEN ? = 'HEALTHY' THEN 0 ELSE consecutive_failures + 1 END, updated_at = unixepoch() WHERE server_id = ?;

-- name: ServerPrivateNetworksSetHealthHandshake :exec
UPDATE server_private_networks SET health_status = ?, health_error = ?, last_handshake_at = ?, last_health_check_at = unixepoch(), consecutive_failures = CASE WHEN ? = 'HEALTHY' THEN 0 ELSE consecutive_failures + 1 END, updated_at = unixepoch() WHERE server_id = ?;

-- name: ServerPrivateNetworksBeginRotation :exec
UPDATE server_private_networks SET rotation_state = ? WHERE server_id = ?;

-- name: ServerPrivateNetworksFinishRotation :exec
UPDATE server_private_networks SET public_key = ?, rotation_state = ?, config_version = config_version + 1, updated_at = unixepoch() WHERE server_id = ?;

-- name: ServerPrivateNetworksFailRotation :exec
UPDATE server_private_networks SET rotation_state = ? WHERE server_id = ?;
