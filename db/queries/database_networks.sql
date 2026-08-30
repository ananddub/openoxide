-- Database networks (shared docker networks for managed databases).

-- name: DatabaseNetworksList :many
SELECT id, name, docker_network_name, description, external, server_id, created_at, updated_at
               FROM database_networks
               ORDER BY name ASC, id ASC;

-- name: DatabaseNetworksListGlobal :many
SELECT id, name, docker_network_name, description, external, server_id, created_at, updated_at
               FROM database_networks
               WHERE server_id IS NULL
               ORDER BY name ASC, id ASC;

-- name: DatabaseNetworksListByServer :many
SELECT id, name, docker_network_name, description, external, server_id, created_at, updated_at
               FROM database_networks
               WHERE server_id = ?
               ORDER BY name ASC, id ASC;

-- name: DatabaseNetworksGetById :one
SELECT id, name, docker_network_name, description, external, server_id, created_at, updated_at
               FROM database_networks
               WHERE id = ?;

-- name: DatabaseNetworksCreateGlobal :one
INSERT INTO database_networks (name, docker_network_name, description, external, server_id)
               VALUES (?, ?, ?, ?, NULL)
               RETURNING id;

-- name: DatabaseNetworksCreateForServer :one
INSERT INTO database_networks (name, docker_network_name, description, external, server_id)
               VALUES (?, ?, ?, ?, ?)
               RETURNING id;

-- name: DatabaseNetworksUpdateGlobal :exec
UPDATE database_networks
               SET name = ?,
                   docker_network_name = ?,
                   description = ?,
                   external = ?,
                   server_id = NULL,
                   updated_at = unixepoch()
               WHERE id = ?;

-- name: DatabaseNetworksUpdateForServer :exec
UPDATE database_networks
               SET name = ?,
                   docker_network_name = ?,
                   description = ?,
                   external = ?,
                   server_id = ?,
                   updated_at = unixepoch()
               WHERE id = ?;

-- name: DatabaseNetworksDelete :exec
DELETE FROM database_networks WHERE id = ?;
