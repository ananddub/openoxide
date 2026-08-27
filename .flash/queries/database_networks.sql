-- Extracted from src/db/repository/database_networks.rs. Do not edit repository SQL directly.

-- name: DatabaseNetworksList :many
SELECT id, name, docker_network_name, description, external, server_id, created_at, updated_at
               FROM database_networks
               ORDER BY name ASC, id ASC;

-- name: DatabaseNetworksListByServer :many
SELECT id, name, docker_network_name, description, external, server_id, created_at, updated_at
               FROM database_networks
               WHERE (? IS NULL AND server_id IS NULL) OR server_id = ?
               ORDER BY name ASC, id ASC;

-- name: DatabaseNetworksGetById :one
SELECT id, name, docker_network_name, description, external, server_id, created_at, updated_at
               FROM database_networks
               WHERE id = ?;

-- name: DatabaseNetworksCreate :exec
INSERT INTO database_networks (name, docker_network_name, description, external, server_id)
               VALUES (?, ?, ?, ?, ?);

-- name: DatabaseNetworksUpdate :exec
UPDATE database_networks
               SET name = COALESCE(?, name),
                   docker_network_name = COALESCE(?, docker_network_name),
                   description = COALESCE(?, description),
                   external = COALESCE(?, external),
                   server_id = COALESCE(?, server_id)
               WHERE id = ?;

-- name: DatabaseNetworksDelete :exec
DELETE FROM database_networks WHERE id = ?;
