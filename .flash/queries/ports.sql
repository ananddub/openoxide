-- Extracted from src/db/repository/ports.rs. Do not edit repository SQL directly.

-- name: PortsGetAll :many
SELECT id AS id, published_port AS published_port, target_port AS target_port, protocol AS protocol, publish_mode AS publish_mode, service_id AS service_id, created_at AS created_at FROM ports;

-- name: PortsGetById :one
SELECT id AS id, published_port AS published_port, target_port AS target_port, protocol AS protocol, publish_mode AS publish_mode, service_id AS service_id, created_at AS created_at FROM ports WHERE id = ?;

-- name: PortsListByApplication :many
SELECT id AS id, published_port AS published_port,
                      target_port AS target_port, protocol AS protocol,
                      publish_mode AS publish_mode, service_id AS service_id,
                      created_at AS created_at
               FROM ports WHERE service_id = ? ORDER BY id;

-- name: PortsCreateForApplication :one
INSERT INTO ports (published_port, target_port, protocol, publish_mode, service_id)
               VALUES (?, ?, ?, ?, ?)
               RETURNING id, published_port, target_port, protocol, publish_mode, service_id, created_at;

-- name: PortsUpdateForApplication :one
UPDATE ports SET published_port = ?, target_port = ?, protocol = ?, publish_mode = ?
               WHERE id = ? AND service_id = ?
               RETURNING id, published_port, target_port, protocol, publish_mode, service_id, created_at;

-- name: PortsDeleteForApplication :one
DELETE FROM ports WHERE id = ? AND service_id = ? RETURNING id;

-- name: PortsCreate :exec
INSERT INTO ports (published_port, target_port, protocol, publish_mode, service_id, created_at) VALUES (?, ?, ?, ?, ?, ?);

-- name: PortsUpdate :exec
UPDATE ports SET published_port = ?, target_port = ?, protocol = ?, publish_mode = ?, service_id = ?, created_at = ? WHERE id = ?;

-- name: PortsDelete :exec
DELETE FROM ports WHERE id = ?;
